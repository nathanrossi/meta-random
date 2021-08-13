SUMMARY = "sigrok-firmware-fx2lafw - firmware for fx2lafw devices"
HOMEPAGE = "https://sigrok.org/"
SECTION = "hardware/instruments"

LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=751419260aa954499f7abaabaa882bbe"

PV = "0.1.6"

SRC_URI = "https://sigrok.org/download/binary/sigrok-firmware-fx2lafw/sigrok-firmware-fx2lafw-bin-0.1.6.tar.gz"
SRC_URI[md5sum] = "244150187dff38f7baba3346503c827f"
SRC_URI[sha256sum] = "5c134fa93d5f71606f0c909dcf1c10aba9bb55546d1cf012086b127871474d93"

S = "${WORKDIR}/sigrok-firmware-fx2lafw-bin-${PV}"

do_compile () {
    :
}

do_install () {
    install -d ${D}${datadir}/sigrok-firmware
    for i in ${S}/*.fw; do
        install -m 0644 $i ${D}${datadir}/sigrok-firmware/$(basename $i)
    done
}

FILES:${PN} += " \
    ${datadir}/sigrok-firmware/* \
    "

BBCLASSEXTEND = "native nativesdk"
