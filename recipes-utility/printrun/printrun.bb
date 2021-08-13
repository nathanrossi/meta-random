SUMMARY = "Pronterface, Pronsole, and Printcore - Pure Python 3d printing host software"
HOMEPAGE = "https://www.pronterface.com/"
LICENSE = "GPLv3"
SECTION = "3dprinting"

LIC_FILES_CHKSUM = "file://COPYING;md5=d32239bcb673463ab874e80d47fae504"

SRC_URI = "git://github.com/kliment/Printrun;protocol=https"
SRCREV = "1423eb46eb7ee095d1737c5382090b33c61e4b86"

PV = "2.0.0rc5+git${SRCPV}"

S = "${WORKDIR}/git"

inherit setuptools3

DEPENDS += " \
        python3-cython-native \
        python3-pyserial-native \
        "

RDEPENDS:${PN} += " \
        python3-pyserial \
        python3-appdirs \
        python3-xmlrpc \
        python3-html \
        python3-codecs \
        python3-threading \
        python3-netserver \
        python3-pydoc \
        "

# install scripts directly into bindir
DISTUTILS_INSTALL_ARGS += "--install-scripts=${bindir}"

PACKAGES += "${PN}-data"

FILES:${PN} += "${datadir}/metainfo/*"
FILES:${PN}-data += "${datadir}/pronterface"

BBCLASSEXTEND = "native nativesdk"
