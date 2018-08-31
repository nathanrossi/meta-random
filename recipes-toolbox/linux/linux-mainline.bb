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
PV = "4.18-pre"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

kernel_do_configure_prepend() {
    cp ${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG} ${B}/.config

    # iptables/etc modules
    echo "CONFIG_IP_NF_IPTABLES=m" >> ${B}/.config
    echo "CONFIG_IP_NF_MATCH_ECN=m" >> ${B}/.config
    echo "CONFIG_IP_NF_MATCH_TTL=m" >> ${B}/.config
    echo "CONFIG_IP_NF_FILTER=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_REJECT=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_MASQUERADE=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_REDIRECT=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_NETMAP=m" >> ${B}/.config
    echo "CONFIG_IP_NF_MANGLE=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_ECN=m" >> ${B}/.config
    echo "CONFIG_IP_NF_TARGET_CLUSTERIP=m" >> ${B}/.config
    echo "CONFIG_IP_NF_RAW=m" >> ${B}/.config
    echo "CONFIG_IP_NF_ARPTABLES=m" >> ${B}/.config
    echo "CONFIG_IP_NF_ARPFILTER=m" >> ${B}/.config
    echo "CONFIG_IP_NF_ARP_MANGLE=m" >> ${B}/.config
}

KBUILD_DEFCONFIG_aarch64 = "defconfig"
COMPATIBLE_MACHINE_raspberrypi3-b-plus = ".*"
KBUILD_DEFCONFIG_raspberrypi0-wifi ?= "bcm2835_defconfig"
COMPATIBLE_MACHINE_raspberrypi0-wifi = ".*"
