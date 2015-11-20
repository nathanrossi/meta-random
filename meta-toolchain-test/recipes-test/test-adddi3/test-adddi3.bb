DESCRIPTION = "test-adddi3"
LICENSE = "MIT"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = "file://adddi3-tests.c"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

do_compile() {
	$CC -o ${B}/test-adddi3 ${WORKDIR}/adddi3-tests.c
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/test-adddi3 ${D}${bindir}/test-adddi3
}

FILES_${PN} += "${bindir}/test-adddi3"

