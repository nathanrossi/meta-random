require recipes-bsp/u-boot/u-boot-common.inc
require recipes-bsp/u-boot/u-boot.inc

LIC_FILES_CHKSUM = "file://Licenses/README;md5=5a7450c57ffe5ae63fd732446b988025"

SRCREV = "e4b6ebd3de982ae7185dbf689a030e73fd06e0d2"

DEPENDS += "bc-native dtc-native python3-setuptools-native"

FILESEXTRAPATHS:prepend := "${THISDIR}/u-boot:"

SRC_URI:append:sg3100 = " \
    file://0001-ddr-marvell-a38x-Import-master-DDR3-4-training-code.patch \
    file://0002-Add-support-for-Netgate-SG3100.patch \
    "

COMPATIBLE_MACHINE = "sg3100"
