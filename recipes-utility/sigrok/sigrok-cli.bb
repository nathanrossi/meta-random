SUMMARY = "sigrok-cli - Command line UI"
HOMEPAGE = "https://sigrok.org/"
SECTION = "hardware/instruments"

LICENSE = "GPLv3"
LIC_FILES_CHKSUM = "file://COPYING;md5=d32239bcb673463ab874e80d47fae504"

SRC_URI = "git://sigrok.org/sigrok-cli"
SRCREV = "a30c837a1a2562f60b329be70b1ca6f7e9a5c2e5"

S = "${WORKDIR}/git"

DEPENDS += "glib-2.0 libsigrok libsigrokdecode"

inherit autotools pkgconfig

FILES_${PN} += " \
    ${datadir}/icons/hicolor/* \
    "

BBCLASSEXTEND = "native nativesdk"
