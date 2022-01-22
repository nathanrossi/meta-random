SUMMARY = "Raspberry Pi USB booting code"
HOMEPAGE = "https://github.com/raspberrypi/usbboot"
SECTION = "bsp"

LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=e3fc50a88d0a364313df4b21ef20c29e"

SRC_URI = "git://github.com/raspberrypi/usbboot;protocol=https;branch=master"
SRCREV = "d3760e119385a179765f43a50a8e051a44127c25"
PV = "0+git${SRCPV}"

S = "${WORKDIR}/git"

DEPENDS += "libusb"

do_configure[noexec] = "1"

# fmemopen support needed
CFLAGS:append:libc-musl = " -D_GNU_SOURCE"

do_compile() {
    oe_runmake clean
    oe_runmake CC="${BUILD_CC}" bin2c
    oe_runmake CC="${CC} ${CFLAGS} ${LDFLAGS}" rpiboot
}

do_install() {
    install -d ${D}${bindir}
    install -m 0755 ${B}/rpiboot ${D}${bindir}/rpiboot
}

BBCLASSEXTEND = "native nativesdk"
