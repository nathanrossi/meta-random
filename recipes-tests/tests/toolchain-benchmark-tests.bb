DESCRIPTION = "Toolchain Benchmark test programs"
LICENSE = "MIT"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
FILESEXTRAPATHS_append := "${THISDIR}/../uio/files:"
SRC_URI = " \
		file://compiler-bench.py \
		file://test-helloworld.c \
		file://test-helloworld.cpp \
		file://test-uio.c \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

# depend on zlib to get its source downloaded
DEPENDS += "zlib"

# run time test dependencies
RDEPENDS_${PN} += " \
		python3-core python3-io python3-datetime python3-subprocess python3-shell \
		tar xz \
		make \
		binutils binutils-symlinks \
		cpp cpp-symlinks \
		gcc gcc-symlinks \
		g++ g++-symlinks \
		libstdc++ \
		"

do_compile[noexec] = "1"

do_install() {
	install -d ${D}${datadir}/compiler-bench
	install -m 0755 ${WORKDIR}/compiler-bench.py ${D}${datadir}/compiler-bench/compiler-bench
	install -m 0644 ${WORKDIR}/test-helloworld.c ${D}${datadir}/compiler-bench/test-helloworld.c
	install -m 0644 ${WORKDIR}/test-helloworld.cpp ${D}${datadir}/compiler-bench/test-helloworld.cpp
	install -m 0644 ${WORKDIR}/test-uio.c ${D}${datadir}/compiler-bench/test-uio.c

	install -m 0644 ${DL_DIR}/zlib-1.2.11.tar.xz ${D}${datadir}/compiler-bench/zlib-1.2.11.tar.xz
}

FILES_${PN} += " \
		${datadir}/compiler-bench \
		"

