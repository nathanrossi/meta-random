require recipes-bsp/u-boot/u-boot-common.inc
require recipes-bsp/u-boot/u-boot.inc

DEPENDS += "bc-native dtc-native python3-setuptools-native"

SRC_URI = "git://github.com/smaeul/u-boot;protocol=https;branch=bl808"
SRCREV = "ac473b19a826b11d851b28bc664a5bb28cd91383"

PV = "2023.03+bl808+git${SRCPV}"

FILESEXTRAPATHS:prepend := "${THISDIR}/u-boot:"
SRC_URI:append:ox64 = " file://0001-Revert-bl808-Use-XIP-for-M0.patch "

COMPATIBLE_MACHINE = "ox64"

do_deploy:append() {
    if [ -e ${B}/bl808_d0_defconfig ]; then
        install -D -m 644 \
            ${B}/bl808_d0_defconfig/arch/riscv/dts/bl808-d0-ox64.dtb \
            ${DEPLOYDIR}/u-boot-bl808-d0-ox64.dtb
    fi
}
