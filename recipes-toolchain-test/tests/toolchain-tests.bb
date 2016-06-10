DESCRIPTION = "Toolchain test programs"
LICENSE = "MIT"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = " \
		file://test-adddi3.c \
		file://test-cpp-so.cpp \
		file://test-mutex.c \
		file://test-pthread.c \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

do_compile() {
	$CC -o ${B}/test-adddi3 ${WORKDIR}/test-adddi3.c
	$CXX -o ${B}/test-cpp-so ${WORKDIR}/test-cpp-so.cpp
	$CC -lpthread -lgcc_s -o ${B}/test-mutex ${WORKDIR}/test-mutex.c
	$CC -lpthread -o ${B}/test-pthread ${WORKDIR}/test-pthread.c
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/test-adddi3 ${D}${bindir}/test-adddi3
	install -m 0755 ${B}/test-cpp-so ${D}${bindir}/test-cpp-so
	install -m 0755 ${B}/test-mutex ${D}${bindir}/test-mutex
	install -m 0755 ${B}/test-pthread ${D}${bindir}/test-pthread
}

FILES_${PN} += " \
		${bindir}/test-adddi3 \
		${bindir}/test-cpp-so \
		${bindir}/test-mutex \
		${bindir}/test-pthread \
		"
