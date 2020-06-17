SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=6bc538ed5bd9a7fc9398086aedcd7e46"

inherit kernel

# for ORC
DEPENDS += "elfutils-native"

# disable kernel-base depending on image, other mechanisms are used to ship the kernel
RDEPENDS_${KERNEL_PACKAGE_NAME}-base = ""

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "69119673bd50b176ded34032fadd41530fb5af21"
PV = "5.8-rc1+git${SRCPV}"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

python do_generate_config() {
    with open(d.expand("${B}/.config"), "w") as f:
        with open(d.expand("${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG}"), "r") as src:
            f.write(src.read())

        # iptables/etc modules
        f.write("CONFIG_IP_NF_IPTABLES=m\n")
        f.write("CONFIG_IP_NF_MATCH_ECN=m\n")
        f.write("CONFIG_IP_NF_MATCH_TTL=m\n")
        f.write("CONFIG_IP_NF_FILTER=m\n")
        f.write("CONFIG_IP_NF_TARGET_REJECT=m\n")
        f.write("CONFIG_IP_NF_TARGET_MASQUERADE=m\n")
        f.write("CONFIG_IP_NF_TARGET_REDIRECT=m\n")
        f.write("CONFIG_IP_NF_TARGET_NETMAP=m\n")
        f.write("CONFIG_IP_NF_MANGLE=m\n")
        f.write("CONFIG_IP_NF_TARGET_ECN=m\n")
        f.write("CONFIG_IP_NF_TARGET_CLUSTERIP=m\n")
        f.write("CONFIG_IP_NF_RAW=m\n")
        f.write("CONFIG_IP_NF_ARPTABLES=m\n")
        f.write("CONFIG_IP_NF_ARPFILTER=m\n")
        f.write("CONFIG_IP_NF_ARP_MANGLE=m\n")
}
addtask generate_config before do_configure after do_unpack

#KBUILD_DEFCONFIG_arm = "multi_v7_defconfig"

KBUILD_DEFCONFIG_aarch64 = "defconfig"
KBUILD_DEFCONFIG_arm_rpi = "bcm2835_defconfig"
COMPATIBLE_MACHINE_rpi = ".*"

KBUILD_DEFCONFIG_raspberrypi0-wifi ?= "bcm2835_defconfig"
COMPATIBLE_MACHINE_raspberrypi0-wifi = ".*"

python do_generate_config_append_rpi () {
    with open(d.expand("${B}/.config"), "a") as f:
        f.write("CONFIG_STAGING=y\n")
        f.write("CONFIG_BCM_VIDEOCORE=y\n")
        f.write("CONFIG_BCM2835_VCHIQ=m\n")
        f.write("CONFIG_VIDEO_BCM2835=m\n")

        # hwrng
        f.write("CONFIG_HW_RANDOM=y\n")
        f.write("CONFIG_HW_RANDOM_BCM2835=y\n")
        f.write("CONFIG_HW_RANDOM_IPROC_RNG200=y\n")

        # needed for usb serial devices
        f.write("CONFIG_USB_SERIAL=y\n")
        f.write("CONFIG_USB_SERIAL_GENERIC=y\n")
        f.write("CONFIG_USB_SERIAL_ARK3116=m\n")
        f.write("CONFIG_USB_SERIAL_BELKIN=m\n")
        f.write("CONFIG_USB_SERIAL_CH341=m\n")
        f.write("CONFIG_USB_SERIAL_CP210X=m\n")
        f.write("CONFIG_USB_SERIAL_FTDI_SIO=m\n")
        f.write("CONFIG_USB_SERIAL_F81232=m\n")
        f.write("CONFIG_USB_SERIAL_IPW=m\n")
        f.write("CONFIG_USB_SERIAL_PL2303=m\n")
        f.write("CONFIG_USB_SERIAL_OTI6858=m\n")
        f.write("CONFIG_USB_SERIAL_QUALCOMM=m\n")
        f.write("CONFIG_USB_SERIAL_SPCP8X5=m\n")
        f.write("CONFIG_USB_SERIAL_SIERRAWIRELESS=m\n")
        f.write("CONFIG_USB_SERIAL_TI=m\n")
        f.write("CONFIG_USB_SERIAL_WWAN=m\n")
        f.write("CONFIG_USB_SERIAL_OPTION=m\n")
        f.write("CONFIG_USB_ACM=m\n")
}

python do_generate_config_append_qemuarm () {
    with open(d.expand("${B}/.config"), "a") as f:
        f.write("CONFIG_DEVTMPFS=y\n")
        f.write("CONFIG_THUMB=y\n")
        f.write("CONFIG_PCI=y\n")
        f.write("CONFIG_PCI_VERSATILE=y\n")
}
KBUILD_DEFCONFIG_qemuarm = "versatile_defconfig"
COMPATIBLE_MACHINE_qemuarm = ".*"
KERNEL_DEVICETREE_qemuarm = "versatile-pb.dtb"

python do_generate_config_append_toolbox-x64 () {
    with open(d.expand("${B}/.config"), "a") as f:
        f.write("CONFIG_EFI_STUB=y\n")
        # allow cpu to be trusted to provide entropy
        f.write("CONFIG_RANDOM_TRUST_CPU=y\n")
}
KBUILD_DEFCONFIG_toolbox-x64 = "x86_64_defconfig"
COMPATIBLE_MACHINE_toolbox-x64 = ".*"
