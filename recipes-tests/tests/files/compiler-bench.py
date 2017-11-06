#!/usr/bin/env python3

import os
import sys
import time
import subprocess
import tempfile

def target_compiler(output, files):
	if any(i.endswith(".cpp") for i in files):
		cc = ["g++"]
		if "CXX" in os.environ:
			cc = [i for i in os.environ["CXX"].split(" ") if len(i) != 0]
	else:
		cc = ["gcc"]
		if "CC" in os.environ:
			cc = [i for i in os.environ["CC"].split(" ") if len(i) != 0]
	args = []
	args += cc
	args += ["-o", output]
	args += files
	return args

def simple_compiler_benchmarks(count = 100):
	benchmarks = [
			("hello-world-c", ["test-helloworld.c"]),
			("hello-world-cpp", ["test-helloworld.cpp"]),
			("sample-uio", ["test-uio.c"]),
			]

	for b, files in benchmarks:
		print("running benchmark {0}".format(b))
		data = []
		with tempfile.TemporaryDirectory() as d:
			filename = os.path.join(d, "test.output")
			args = target_compiler(filename, files)
			for i in range(count):
				print("cycle {0}".format(i))

				t_start = time.perf_counter()
				r = subprocess.run(args)
				t_end = time.perf_counter()

				# check successful completion
				if r.returncode != 0:
					raise Exception("Process execution failed, returncode = {0}".format(r.returncode))

				took = t_end - t_start
				print("cycle {0}, took {1}s".format(i, took))
				data.append(took)

				# clean up output
				os.remove(filename)

		print("========== OUTPUT DATA ({0}) ==========".format(b))
		print(repr(data))
		print("========== OUTPUT DATA ({0}) ==========".format(b))

def zlib_compiler_benchmark(count = 100):
	data = []
	for i in range(count):
		with tempfile.TemporaryDirectory() as d:
			# extract the source
			subprocess.run(["tar", "-xf", os.path.abspath("zlib-1.2.11.tar.xz")], cwd = d)
			# configure
			zlibbuild = os.path.join(d, "zlib-1.2.11")
			subprocess.run([os.path.join(zlibbuild, "configure"), "--shared"], cwd = zlibbuild)
			# setup args
			args = ["make", "shared"]

			# time the make shared
			t_start = time.perf_counter()
			r = subprocess.run(args, cwd = zlibbuild)
			t_end = time.perf_counter()

			# check successful completion
			if r.returncode != 0:
				raise Exception("Process execution failed, returncode = {0}".format(r.returncode))

			took = t_end - t_start
			print("cycle {0}, took {1}s".format(i, took))
			data.append(took)

	print("========== OUTPUT DATA (zlib) ==========")
	print(repr(data))
	print("========== OUTPUT DATA (zlib) ==========")


if __name__ == "__main__":
	runs = int(sys.argv[1]) if len(sys.argv) > 1 else 1
	simple_compiler_benchmarks(runs)
	zlib_compiler_benchmark(runs)

