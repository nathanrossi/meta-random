#!/usr/bin/env python3
import sys
import os
import fcntl
import socket
import select
import struct
import subprocess

if os.environ.get("DEBUG", None) == "1":
	def debug(f, *args):
		print(("debug: " + f).format(*args))
else:
	def debug(f, *args):
		pass

class ClientSession:
	def __init__(self, sock):
		self.sock = sock
		self.command = []
		self.process = None
		self.stdout = None
		self.stderr = None

		self._buf = bytearray()

	def read(self):
		try:
			while True:
				data = self.sock.recv(1024)
				if data is None or len(data) == 0:
					self.sock.close()
					self.sock = None
					if self.process is not None:
						self.process.terminate()
						self.process.wait()
						self.process = None
					return
				debug("server:client: got data for session {}", len(data))
				self.recv(data)
		except BlockingIOError:
			return

	def recv(self, data):
		self._buf += data
		while len(self._buf) >= 8:
			plen, ptype = struct.unpack_from("!II", self._buf)
			if len(self._buf) - 8 < plen:
				# not enough for frame
				return

			debug("server:client: got message {}, {}", plen, ptype)

			if ptype == 0xff: # arg
				self.command.append(self._buf[8:8 + plen].decode("utf-8"))
				debug("server:client: got arg '{}'", self.command[-1])
			elif ptype == 0xfe: # run
				debug("running command '{}'".format(repr(self.command)))
				self.process = subprocess.Popen(self.command, stdout = subprocess.PIPE, stderr = subprocess.PIPE, stdin = subprocess.PIPE)
				self.stdout = self.process.stdout
				self.stderr = self.process.stderr
				fl = fcntl.fcntl(self.stdout, fcntl.F_GETFL)
				fcntl.fcntl(self.stdout, fcntl.F_SETFL, fl | os.O_NONBLOCK)
				fl = fcntl.fcntl(self.stderr, fcntl.F_GETFL)
				fcntl.fcntl(self.stderr, fcntl.F_SETFL, fl | os.O_NONBLOCK)
			elif ptype == 0x00: # stdin
				if self.process is not None:
					self.process.stdin.write(self._buf[8:8 + plen])
			elif ptype == 0x10: # stdin
				if self.process is not None:
					self.process.stdin.close()

			# clear frame
			self._buf = self._buf[8 + plen:]

	def input(self, fd):
		pipeid = 0 if fd is self.stdout else 1
		pipe = self.stdout if fd is self.stdout else self.stderr
		data = pipe.read(1024)
		if data is None or len(data) == 0:
			if fd is self.stdout:
				self.stdout = None
			else:
				self.stderr = None
			self.check_completed()

		if self.sock is not None:
			debug("server:client: sent {} bytes on pipe {}", len(data), pipeid)
			self.sock.send(struct.pack("!II", len(data), pipeid) + data)

	def check_completed(self):
		if self.stdout is None and self.stderr is None:
			debug("server:client: process pipes terminated")
			self.process.wait()
			debug("server:client: process done {}", self.process.returncode)
			self.sock.send(struct.pack("!III", 4, 0xfe, self.process.returncode))
			self.sock.close()
			self.sock = None
			self.process = None

class Server:
	def __init__(self, port):
		self.port = port
		self._sock = None

		self.sessions = []

	def open(self):
		self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		self._sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
		self._sock.bind(("", self.port))
		self._sock.listen(1)
		debug("server: listening")

	def close(self):
		self._sock.close()
		self._sock = None

	def wait(self):
		while True:
			rfds = [self._sock]
			for s in self.sessions:
				if s.sock is None:
					continue

				rfds.append(s.sock)
				if s.stdout is not None:
					rfds.append(s.stdout)
				if s.stderr is not None:
					rfds.append(s.stderr)

			debug("server: select on {}", repr(rfds))
			rd, _, _ = select.select(rfds, [], [])

			for fd in rd:
				if self._sock is fd:
					conn, conn_addr = self._sock.accept()
					conn.setblocking(0)
					rfds.append(conn)
					self.sessions.append(ClientSession(conn))
					debug("server: client session {}", repr(conn_addr))
				else:
					cleanup = []
					for s in self.sessions:
						if s.sock is fd:
							s.read()
						elif s.stdout is fd or s.stderr is fd:
							s.input(fd)

						if s.process is None and s.sock is None:
							cleanup.append(s)

					for c in cleanup:
						debug("server: cleaned up client session")
						self.sessions.remove(c)

class Client:
	def __init__(self, host, port, command):
		self.host = host
		self.port = port
		self.command = command
		self._sock = None
		self._buf = bytearray()

	def open(self):
		self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		self._sock.connect((self.host, self.port))
		self.send_command()
		# nonblocking stdin
		fcntl.fcntl(sys.stdin, fcntl.F_SETFL, fcntl.fcntl(sys.stdin, fcntl.F_GETFL) | os.O_NONBLOCK)

	def close(self):
		self._sock.close()
		self._sock = None
		self._buf = bytearray()

	def send_command(self):
		for arg in self.command:
			debug("client: sent arg '{}'", arg)
			self._sock.send(struct.pack("!II", len(arg), 0xff) + arg.encode("utf-8"))
		debug("client: sent run")
		self._sock.send(struct.pack("!II", 0, 0xfe))

	def send_input_closed(self):
		self._sock.send(struct.pack("!II", 0, 0x10))

	def send_input(self, data):
		self._sock.send(struct.pack("!II", len(data), 0x00) + data)

	def wait(self):
		rfds = [self._sock, sys.stdin]
		while True:
			rd, _, _ = select.select(rfds, [], [])
			for fd in rd:
				try:
					if self._sock is fd:
						while True:
							data = self._sock.recv(1024)
							if data is None or len(data) == 0:
								break
							self.recv(data)
					elif sys.stdin is fd:
						while True:
							data = os.read(sys.stdin.fileno(), 1024)
							if len(data) != 0:
								self.send_input(data)
							if data is None or len(data) == 0:
								rfds.pop(rfds.index(sys.stdin))
								self.send_input_closed()
								break
				except BlockingIOError:
					pass

	def recv(self, data):
		self._buf += data
		while len(self._buf) >= 8:
			plen, ptype = struct.unpack_from("!II", self._buf)
			if len(self._buf) - 8 < plen:
				# not enough for frame
				return

			if ptype == 0x00:
				sys.stdout.write(self._buf[8:8 + plen].decode())
			elif ptype == 0x01:
				sys.stderr.write(self._buf[8:8 + plen].decode())
			elif ptype == 0xfe:
				# program done
				ret = struct.unpack_from("!I", self._buf, 8)
				sys.exit(ret)

			# clear frame
			self._buf = self._buf[8 + plen:]

def main(args):
	if len(args) >= 2 and args[1] == "-s" or args[1] == "-sd":
		if args[1] == "-sd":
			if os.fork() != 0:
				sys.exit(0)

			# fork with second sid'd child
			os.setsid()
			if os.fork() != 0:
				sys.exit(0)

			# open /dev/null for stdio
			dnfd = os.open(os.devnull, os.O_RDWR)
			for i in [0, 1, 2]:
				os.close(i)
				os.dup2(dnfd, i)

		s = Server(2222)
		s.open()
		s.wait()
		s.close()
	else:
		if len(args) < 2:
			sys.exit(-1)
		c = Client(args[1], 2222, args[2:])
		c.open()
		c.wait()
		c.close()

try:
	main(sys.argv)
except KeyboardInterrupt:
	sys.exit(0)

