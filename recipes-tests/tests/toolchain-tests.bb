DESCRIPTION = "Toolchain test programs"
LICENSE = "MIT"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = " \
		file://test-helloworld.c \
		file://test-adddi3.c \
		file://test-cpp-so.cpp \
		file://test-mutex.c \
		file://test-pthread.c \
		file://test-gcc-atomic.c \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

do_compile() {
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-helloworld ${WORKDIR}/test-helloworld.c
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-adddi3 ${WORKDIR}/test-adddi3.c
	$CXX $TARGET_CXXFLAGS $TARGET_LDFLAGS -o ${B}/test-cpp-so ${WORKDIR}/test-cpp-so.cpp
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-mutex ${WORKDIR}/test-mutex.c -lgcc_s -lpthread
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-pthread ${WORKDIR}/test-pthread.c -lpthread
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-gcc-atomic ${WORKDIR}/test-gcc-atomic.c
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/test-helloworld ${D}${bindir}/test-helloworld
	install -m 0755 ${B}/test-adddi3 ${D}${bindir}/test-adddi3
	install -m 0755 ${B}/test-cpp-so ${D}${bindir}/test-cpp-so
	install -m 0755 ${B}/test-mutex ${D}${bindir}/test-mutex
	install -m 0755 ${B}/test-pthread ${D}${bindir}/test-pthread
	install -m 0755 ${B}/test-gcc-atomic ${D}${bindir}/test-gcc-atomic
}

FILES_${PN} += " \
		${bindir}/test-helloworld \
		${bindir}/test-adddi3 \
		${bindir}/test-cpp-so \
		${bindir}/test-mutex \
		${bindir}/test-pthread \
		${bindir}/test-gcc-atomic \
		"

QEMU_USER_TESTS = "test-helloworld test-adddi3 test-cpp-so test-mutex test-pthread test-gcc-atomic"

