DESCRIPTION = "Simple rexec"
LICENSE = "MIT"

SRC_URI = "file://rexec.py"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

do_compile[noexec] = "1"

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${WORKDIR}/rexec.py ${D}${bindir}/rexec
}

RDEPENDS_${PN} += "python3"
FILES_${PN} += " \
		${bindir}/rexec \
		"

BBCLASSEXTEND += "native"
