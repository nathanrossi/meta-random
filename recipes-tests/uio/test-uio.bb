DESCRIPTION = "UIO test program"
LICENSE = "MIT"

SRC_URI = " \
		file://test-uio.c \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

do_compile() {
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-uio ${WORKDIR}/test-uio.c
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/test-uio ${D}${bindir}/test-uio
}

FILES:${PN} += " \
		${bindir}/test-uio \
		"

