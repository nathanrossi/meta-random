SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=bbea815ee2795b2f4230826c0c6b8814"

inherit kernel

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "${AUTOREV}"
PV = "4.17-rc"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"
SRC_URI_append = " \
        file://0001-ARM-dts-bcm283x-Fix-PWM-pin-assignment.patch \
        file://0002-ARM-dts-bcm2837-Add-missing-GPIOs-of-Expander.patch \
        file://0003-dt-bindings-bcm-Add-Raspberry-Pi-3-B.patch \
        file://0004-ARM-dts-bcm2837-Add-Raspberry-Pi-3-B.patch \
        file://0005-arm64-dts-broadcom-Add-reference-to-Raspberry-Pi-3-B.patch \
        file://0006-ARM-bcm2835_defconfig-Enable-LAN78XX-driver.patch \
        file://0007-ARM-bcm2835_defconfig-Enable-VCHIQ-driver.patch \
        file://0008-ARM-multi_v7_defconfig-Enable-LAN-and-BT-for-RPi-3-B.patch \
        file://0009-arm64-defconfig-Enable-LAN-and-BT-support-for-RPi-3-.patch \
        file://0010-lan78xx-Read-MAC-address-from-DT-if-present.patch \
        file://0011-lan78xx-Read-LED-states-from-Device-Tree.patch \
        file://0012-dt-bindings-Document-the-DT-bindings-for-lan78xx.patch \
        "

kernel_do_configure_prepend() {
    cp ${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG} ${B}/.config
}

COMPATIBLE_MACHINE_raspberrypi3-64 = ".*"
KBUILD_DEFCONFIG_raspberrypi3-64 = "defconfig"
KERNEL_DEVICETREE_raspberrypi3-64 = "broadcom/bcm2837-rpi-3-b.dtb broadcom/bcm2837-rpi-3-b-plus.dtb"

