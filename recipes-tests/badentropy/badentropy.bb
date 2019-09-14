DESCRIPTION = ""
LICENSE = "MIT"

LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

FILESEXTRAPATH_prepend := "${THISDIR}/files:"
SRC_URI = " \
		file://main.c \
		"

S = "${WORKDIR}"

do_compile() {
	${CC} ${CFLAGS} ${LDFLAGS} -o badentropy main.c
}

do_install() {
	install -d ${D}${bindir}
	install -m 755 ${B}/badentropy ${D}/${bindir}

	install -d ${D}${sysconfdir}/init.d
	ln -s ${bindir}/badentropy ${D}${sysconfdir}/init.d/badentropy
}

FILES_${PN} += " \
	${bindir}/badentropy \
	"

inherit update-rc.d

INITSCRIPT_NAME = "badentropy"
INITSCRIPT_PARAMS = "defaults 3"

