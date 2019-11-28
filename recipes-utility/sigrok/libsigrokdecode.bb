SUMMARY = "libsigrokdecode - Protocol decoder lib"
HOMEPAGE = "https://sigrok.org/"
SECTION = "hardware/instruments"

LICENSE = "GPLv3"
LIC_FILES_CHKSUM = "file://COPYING;md5=d32239bcb673463ab874e80d47fae504"

SRC_URI = "git://sigrok.org/libsigrokdecode"
SRCREV = "577af027774c422a1d5a73b7a8d1da03caa4e068"

S = "${WORKDIR}/git"

DEPENDS += "glib-2.0 python3"

inherit autotools pkgconfig

BBCLASSEXTEND = "native nativesdk"
