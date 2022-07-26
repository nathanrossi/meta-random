SECTION = "bsp"
DEPENDS = "readline libusb"

LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://COPYING;md5=3178b603d6560dc009468f671919776f"

SRC_URI = "git://github.com/dlbeer/mspdebug;protocol=https;branch=master"
SRCREV = "985b390ba22f4229aeca9f02e273a54eb4a76a9a"

S = "${WORKDIR}/git"
B = "${S}"

do_configure[noexec] = "1"

do_compile() {
    oe_runmake all
}

do_install() {
    oe_runmake install PREFIX=${prefix} DESTDIR=${D}
}

BBCLASSEXTEND = "native nativesdk"
