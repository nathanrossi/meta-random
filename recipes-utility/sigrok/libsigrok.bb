SUMMARY = "libsigrok - hardware access and backend lib"
HOMEPAGE = "https://sigrok.org/"
SECTION = "hardware/instruments"

LICENSE = "GPLv3"
LIC_FILES_CHKSUM = "file://COPYING;md5=d32239bcb673463ab874e80d47fae504"

SRC_URI = "git://sigrok.org/libsigrok"
SRCREV = "4be5746d1dd2796aa10f0c45440005d28a554901"

S = "${WORKDIR}/git"

DEPENDS += "glib-2.0 libzip"
DEPENDS += "libusb libftdi"
#DEPENDS += "libserialport librevisa libgpib libieee1284"

inherit autotools pkgconfig

RDEPENDS:${PN} += "sigrok-firmware-fx2lafw"
FILES:${PN} += " \
    ${datadir}/icons/hicolor/* \
    ${datadir}/mime/packages/* \
    "

BBCLASSEXTEND = "native nativesdk"
