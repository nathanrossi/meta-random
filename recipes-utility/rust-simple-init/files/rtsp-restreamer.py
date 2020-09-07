#!/usr/bin/env python3
import sys
import os
import socket
import struct
import select
import re
import urllib.parse

# mpv rtsp://localhost:8080/test.mp4
# ffmpeg -re -i test-kf.mp4 -vcodec copy -an -f rtsp -rtsp_transport tcp rtsp://127.0.0.1:8080/test.mp4

def log(*args):
	print("[rtsp]", *args)

class RtspStream:
	class ResponseCode:
		Continue = (100, "Continue")
		OK = (200, "OK")
		NotFound = (404, "Not Found")
		SessionNotFound = (454, "Not Found")
		UnsupportedTransport = (461, "Unsupported transport")

	def __init__(self, sock, address = None):
		self.address = address
		self.sock = sock
		self.buf = None
		self.cseq = 0

	def recv(self):
		if self.sock is None:
			return
		data = self.sock.recv(4096)
		if data is None or len(data) == 0:
			self.close()
			return

		self.buf = data if self.buf is None else (self.buf + data)
		while True:
			if self.buf is None or len(self.buf) < 4:
				break
			if self.buf[0] == 0x24: # $<n><len>... - inline data
				datalen = struct.unpack_from("!H", self.buf, offset = 2)[0]
				if len(self.buf) < datalen + 4:
					break
				payload = memoryview(self.buf)[4:datalen + 4]
				yield (self.buf[1], payload)
				self.buf = self.buf[datalen + 4:]
			else: # search for message
				headers = self.parse_reqresp_payload(self.buf)
				if headers is None:
					break
				message, fields = self.parse_fields(headers)
				content = None
				clen = int(fields.get("Content-Length", 0))
				if clen > 0: # collect content
					if len(self.buf) < (len(headers) + clen):
						break
					content = self.buf[len(headers):len(headers) + clen]
				self.buf = self.buf[len(headers) + clen:]
				self.cseq = int(fields.get("CSeq", self.cseq))
				yield (None, (self.parse_header(message), fields, content))

	@staticmethod
	def parse_reqresp_payload(data):
		tail = [0xd, 0xa, 0xd, 0xa]
		for i in range(len(data) - (len(tail) - 1)):
			if all((data[i + ti] == t) for ti, t in enumerate(tail)):
				payload = memoryview(data)[0:i + len(tail)]
				return payload
		return None

	@staticmethod
	def parse_header(message):
		parts = message.split()
		if message.startswith("RTSP/1.0"): # server response
			status, statusmsg = int(parts[1]), parts[2]
			for k, v in RtspStream.ResponseCode.__dict__.items():
				if not k.startswith("_") and v[0] == status:
					return v
			return status, statusmsg
		# client request
		return parts[0], urllib.parse.urlparse(parts[1]).path, parts[1]

	@staticmethod
	def parse_fields(data):
		message = bytes(data).decode().split("\r\n")
		fields = {}
		for i in message[1:]:
			if len(i) != 0:
				parts = i.split(": ", 1)
				if len(parts) != 2:
					raise Exception("Got invalid field data")
				fields[parts[0]] = parts[1]
		return message[0], fields

	def send(self, message):
		if self.sock is None:
			return
		try:
			self.sock.send(message)
		except ConnectionResetError:
			self.close()

	def send_substream(self, streamid, payload):
		if self.sock is None:
			return
		header = struct.pack("!BBH", 0x24, streamid, len(payload))
		self.send(header + payload)

	def send_response(self, response = None, fields = None, content = None, cseq = None):
		response = response or RtspStream.ResponseCode.OK
		info = "RTSP/1.0 {} {}\r\n".format(response[0], response[1])
		message = bytearray(info.encode())
		for key, val in (fields or {}).items():
			message += "{}: {}\r\n".format(key, val).encode()
		message += "CSeq: {:d}\r\n".format(cseq or self.cseq).encode()
		if content is not None:
			message += "Content-Length: {:d}\r\n".format(len(content)).encode()
			message += "\r\n".encode()
			message += content
		else:
			message += "\r\n".encode()
		self.send(message)

	def send_request(self, verb, path, fields = None, content = None, cseq = None):
		info = "{} {} RTSP/1.0\r\n".format(verb, self.address._replace(path = path).geturl())
		message = bytearray(info.encode())
		for key, val in (fields or {}).items():
			message += "{}: {}\r\n".format(key, val).encode()
		cseq = cseq or (self.cseq + 1)
		message += "CSeq: {:d}\r\n".format(cseq).encode()
		if content is not None:
			message += "Content-Length: {:d}\r\n".format(len(content)).encode()
			message += "\r\n".encode()
			message += content
		else:
			message += "\r\n".encode()
		self.send(message)

	def wait_response(self):
		while True:
			if self.sock is None:
				return None, None, None
			rfds, _, _ = select.select([self.sock], [], [])
			for s, data in self.recv():
				if s is None:
					return data
				else:
					pass # drop all other data

	def request(self, *args, **kwargs):
		self.send_request(*args, **kwargs)
		return self.wait_response()

	@staticmethod
	def open(url):
		parts = urllib.parse.urlparse(url)
		sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
		sock.connect((parts.hostname, parts.port or 554))
		sock.setblocking(0)
		return RtspStream(sock, address = parts._replace(path = "", params = "", query = "", fragment = ""))

	def close(self):
		if self.sock is None:
			return
		self.sock.close()
		self.sock = None
		self.buf = None

def get_sdp_video_stream(lines):
	groups = []
	remaining = []
	for i in lines:
		if len(i) != 0:
			if i[0] in ["v", "t", "m"] and len(remaining) != 0:
				groups.append(remaining)
				remaining = []
			remaining.append(i)
	if len(remaining) != 0:
		groups.append(remaining)

	for g in groups:
		if g[0].startswith("m=video "): # video element
			parts = g[0].split("=", 1)[1].split()
			rtpmap = int(parts[3])
			control = next((l.split(":", 1)[1] for l in g if l.startswith("a=control:")), None)
			yield (control, rtpmap, g)

class RtspServer:
	def __init__(self, port = None):
		self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		self.sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
		self.sock.setblocking(0)
		self.sock.bind(("", port or 554))
		self.sock.listen()
		self.clients = []
		self.sources = {}
		self.sessionids = 0
		self.sessions = {}

	def new_session(self, client, path):
		self.sessionids += 1
		sessionid = str(self.sessionids)
		self.sessions[sessionid] = (client, [(path, 0)])
		return sessionid

	def add_source(self, url, path = None):
		parts = urllib.parse.urlparse(url)
		client = RtspStream.open(url)
		status, fields, content = client.request("OPTIONS", parts.path)
		if status == RtspStream.ResponseCode.OK:
			public = fields.get("Public").split(", ")
			if "DESCRIBE" not in public or "SETUP" not in public or "PLAY" not in public:
				raise Exception("Cannot access remote stream @ {}, missing DESCRIBE/SETUP/PLAY options".format(url))
		else:
			raise Exception("Cannot access remote stream @ {}, {}".format(url, status))

		status, fields, content = client.request("DESCRIBE", parts.path)
		if status == RtspStream.ResponseCode.OK:
			if fields.get("Content-Type") != "application/sdp":
				raise Exception("Cannot access remote stream @ {}, missing SDP".format(url, status))
			sdp = content
		else:
			raise Exception("Cannot access remote stream @ {}, describe failed with {}".format(url, status))

		# get correct substream
		streams = list(get_sdp_video_stream(sdp.decode().split("\r\n")))
		streampath, streammap, streamattrs = streams[0]
		log("stream", streampath, streammap)

		# rebuild sdp
		sdpparts = ["m=video 0 RTP/AVP {}".format(streammap)]
		sdpparts += [a for a in streamattrs if a.startswith("a=rtpmap:") or a.startswith("a=fmtp:")]
		sdp = ("\r\n".join(sdpparts) + "\r\n").encode()
		log("new sdp", repr(sdp))

		status, fields, content = client.request("SETUP", parts.path + streampath, fields = {
			"Transport" : "RTP/AVP/TCP;interleaved=0-1",
			})
		if status == RtspStream.ResponseCode.OK:
			session = fields.get("Session")
		else:
			raise Exception("Cannot access remote stream @ {}, setup failed with {}".format(url, status))

		# add source to sources
		path = path or parts.path
		self.clients.append(client)
		self.sources[path] = (client, sdp)
		log("created session {}, for restreaming {}".format(repr(session), url))

		status, fields, content = client.request("PLAY", parts.path, fields = {
			"Transport" : "RTP/AVP/TCP;interleaved=0-1",
			"Session" : session,
			})
		if status != RtspStream.ResponseCode.OK:
			raise Exception("Cannot access remote stream @ {}, play failed with {}".format(url, status))

	def handle(self):
		fds = [self.sock]
		for i in self.clients:
			if i.sock is not None:
				fds.append(i.sock)

		rfds, _, _ = select.select(fds, [], [])
		if self.sock in rfds:
			newsock, newaddr = self.sock.accept()
			newsock.setblocking(0)
			log("new client @ {}".format(newaddr))
			self.clients.append(RtspStream(newsock))

		for i in self.clients:
			if i.sock in rfds:
				try:
					for s, data in i.recv():
						if s == 0:
							self.handle_data(i, data)
						elif s == 1:
							pass # ignore rtcp data
						elif s is None:
							self.handle_request(i, data)
				except Exception as e:
					log("exception during handling of client {} - {}".format(i, e))
					i.close()

	def handle_data(self, srcclient, data):
		path = None
		for srcpath, (client, _) in self.sources.items():
			if client == srcclient:
				path = srcpath
				break

		if path is None:
			return

		for session, (client, substreams) in self.sessions.items():
			for ipath, substream in substreams:
				if ipath == path:
					client.send_substream(substream, data)

	def handle_request(self, client, request):
		log("got request from client {}".format(client))
		(verb, path, opath), fields, content = request
		log("fields", repr(fields))
		log("content", repr(content))

		sessionid = fields.get("Session", None)
		log("got {} '{}' (session = {})".format(verb, path, sessionid))
		if verb == "OPTIONS":
			client.send_response(fields = {
				"Public": ", ".join(["DESCRIBE", "SETUP", "ANNOUNCE", "TEARDOWN", "PLAY", "RECORD"]),
				})
		elif verb == "ANNOUNCE":
			self.sources[path] = (client, content)
			log("sdp", repr(content))
			client.send_response()
		elif verb == "DESCRIBE":
			if path not in self.sources:
				client.send_response(RtspStream.ResponseCode.NotFound)
				return

			_, sdp = self.sources.get(path)
			log("sdp", repr(sdp))
			sdp = sdp.decode()

			streaminfo = ""
			streaminfo += "a=recvonly\r\n"
			streaminfo += "a=range:npt=now-\r\n"
			streaminfo += "a=StreamName:string:\"{}\"\r\n".format(path)

			m = re.search("^m=video.*$", sdp, re.MULTILINE)
			streaminfo += m.group(0) + "\r\n"
			m = re.search("^a=rtpmap.*$", sdp, re.MULTILINE)
			if m is not None:
				streaminfo += m.group(0) + "\r\n"
			m = re.search("^a=fmtp.*$", sdp, re.MULTILINE)
			if m is not None:
				streaminfo += m.group(0) + "\r\n"

			log("streaminfo", repr(streaminfo))

			client.send_response(fields = {
				"Content-Base": opath,
				"Content-Type": "application/sdp",
				}, content = streaminfo.encode())
		elif verb == "SETUP":
			transport = fields.get("Transport")
			parts = transport.split(";")
			if parts[0] != "RTP/AVP/TCP":
				client.send_response(RtspStream.ResponseCode.UnsupportedTransport)
				log("client {}, attempted invalid transport '{}'".format(client, parts[0]))
			# TODO: just assume interleaved=0-1 for now
			sessionid = self.new_session(client, path)
			client.send_response(fields = {
				"Session": sessionid,
				"Transport": transport,
				})
		elif verb == "TEARDOWN":
			del self.sessions[sessionid]
			client.send_response()
		elif verb == "RECORD":
			if path not in self.sources:
				client.send_response(RtspStream.ResponseCode.NotFound)
				return
			client.send_response(fields = {
				"Session": sessionid,
				})
		elif verb == "PLAY":
			if sessionid not in self.sessions:
				client.send_response(RtspStream.ResponseCode.SessionNotFound)
				return
			client.send_response(fields = {
				"Session": sessionid,
				# "RTP-Info": "url={};seq={};rtptime={}".format(opath, source.seq or 0, source.timestamp or 0),
				})

args = sys.argv[1:]
port = next((int(a.split("=", 1)[1]) for a in args if a.startswith("--port=")), None)
if port is not None:
	log("starting server on custom port {}".format(port))
server = RtspServer(port = port)
sources = [a for a in args if not a.startswith("--")]
for s in sources:
	log("adding source {}".format(s))
	server.add_source(s)
try:
	while True:
		server.handle()
except KeyboardInterrupt:
	pass

