DESCRIPTION = "Glib test programs"
LICENSE = "MIT"

DEPENDS += "glib-2.0 python3 python3-pygobject"

FILESEXTRAPATHS_append := "${THISDIR}/tests-glib:"
SRC_URI = " \
		file://test-glib-helloworld.c \
		file://test-glib-python-gi.py \
		"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

TARGET_CFLAGS += "-I=${includedir}/glib-2.0 -I=${libdir}/glib-2.0/include"

do_compile() {
	$CC $TARGET_CFLAGS $TARGET_LDFLAGS -o ${B}/test-glib-helloworld ${WORKDIR}/test-glib-helloworld.c -lglib-2.0
	install -m 0755 ${WORKDIR}/test-glib-python-gi.py ${B}/test-glib-python-gi.py
}

do_install() {
	install -d ${D}${bindir}
	install -m 0755 ${B}/test-glib-helloworld ${D}${bindir}/test-glib-helloworld
	install -m 0755 ${B}/test-glib-python-gi.py ${D}${bindir}/test-glib-python-gi.py
}

RDEPENDS_${PN} += "python3 python3-modules python3-pygobject"
FILES_${PN} += " \
		${bindir}/test-glib-helloworld \
		${bindir}/test-glib-python-gi.py \
		"

QEMU_USER_TESTS = "test-glib-helloworld test-glib-python-gi.py"

