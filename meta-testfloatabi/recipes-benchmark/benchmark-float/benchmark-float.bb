DESCRIPTION = ""
LICENSE = "MIT"

LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

FILESEXTRAPATH_prepend := "${THISDIR}/files:"
SRC_URI = " \
		file://benchmark-float.c \
		file://libbenchmark-float.c \
		"

S = "${WORKDIR}"

do_compile() {
	${CC} -shared -o libbenchmark-float.so libbenchmark-float.c
	${CC} -lrt -L${S} -lbenchmark-float -o benchmark-float benchmark-float.c
}

do_install() {
	install -d ${D}${libdir}
	install ${S}/libbenchmark-float.so ${D}/${libdir}

	install -d ${D}${bindir}
	install -m 755 ${S}/benchmark-float ${D}/${bindir}
}

FILES_${PN} += " \
	${bindir}/benchmark-float \
	${libdir}/libbenchmark-float.so \
	"

