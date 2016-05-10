DESCRIPTION = "test-cpp-so"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = "file://cpp-so-tests.cpp"

do_compile() {
	$CXX -o ${B}/${PN} ${WORKDIR}/cpp-so-tests.cpp
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/${PN} ${D}${bindir}/${PN}
}

FILES_${PN} += "${bindir}/${PN}"

