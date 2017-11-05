DESCRIPTION = "Toolchain test programs"
LICENSE = "MIT"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = " \
		file://test-helloworld.c \
		file://test-adddi3.c \
		file://test-bitfields.c \
		file://test-cpp-so.cpp \
		file://test-mutex.c \
		file://test-pthread.c \
		file://test-gcc-atomic.c \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

export CC9 = "${@d.getVar('CC').replace('mcpu=v10.0', 'mcpu=v9.6')}"

do_compile() {
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-helloworld ${WORKDIR}/test-helloworld.c
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-adddi3 ${WORKDIR}/test-adddi3.c
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-bitfields ${WORKDIR}/test-bitfields.c
	$CC9 $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-bitfields-v96 ${WORKDIR}/test-bitfields.c
	$CXX $TARGET_CXXFLAGS $TARGET_LDFLAGS -o ${B}/test-cpp-so ${WORKDIR}/test-cpp-so.cpp
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-mutex ${WORKDIR}/test-mutex.c -lgcc_s -lpthread
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-pthread ${WORKDIR}/test-pthread.c -lpthread
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-gcc-atomic ${WORKDIR}/test-gcc-atomic.c
}

do_install() {
	install -d ${D}${bindir}
	for i in ${B}/*; do
		install -m 0755 $i ${D}${bindir}/$(basename $i)
	done
}

FILES_${PN} += "${bindir}"

QEMU_USER_TESTS = "test-helloworld test-adddi3 test-cpp-so test-mutex test-pthread test-gcc-atomic"

