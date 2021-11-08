SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=6bc538ed5bd9a7fc9398086aedcd7e46"

inherit kernel

# for ORC
DEPENDS += "elfutils-native"

# disable kernel-base depending on image, other mechanisms are used to ship the kernel
RDEPENDS:${KERNEL_PACKAGE_NAME}-base = ""

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "e49d033bddf5b565044e2abe4241353959bc9120"
PV = "5.12-rc6+git${SRCPV}"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

# Build all kernel device trees with overlay/symbol support
KERNEL_DTC_FLAGS = "-@"

python do_generate_config() {
    def append(f, name, val):
        with open(d.expand("${B}/.config"), "a") as f:
            f.write("CONFIG_{}={}\n".format(name, val))

    def config(name, val):
        with open(d.expand("${B}/.config"), "r") as source:
            with open(d.expand("${B}/.config.tmp"), "w") as dest:
                value = "CONFIG_{}".format(name)
                comment = "# " + value

                matched = False
                for l in source:
                    if l.startswith(value) or l.startswith(comment):
                        dest.write("CONFIG_{}={}\n".format(name, val))
                        matched = True
                    else:
                        dest.write(l)

                if not matched: # append if not set
                    dest.write("CONFIG_{}={}\n".format(name, val))

        os.remove(d.expand("${B}/.config"))
        os.rename(d.expand("${B}/.config.tmp"), d.expand("${B}/.config"))

    with open(d.expand("${B}/.config"), "w") as f:
        with open(d.expand("${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG}"), "r") as src:
            f.write(src.read())

    # iptables/etc modules
    config("IP_NF_IPTABLES", "m")
    config("IP_NF_MATCH_ECN", "m")
    config("IP_NF_MATCH_TTL", "m")
    config("IP_NF_FILTER", "m")
    config("IP_NF_TARGET_REJECT", "m")
    config("IP_NF_TARGET_MASQUERADE", "m")
    config("IP_NF_TARGET_REDIRECT", "m")
    config("IP_NF_TARGET_NETMAP", "m")
    config("IP_NF_MANGLE", "m")
    config("IP_NF_TARGET_ECN", "m")
    config("IP_NF_TARGET_CLUSTERIP", "m")
    config("IP_NF_RAW", "m")
    config("IP_NF_ARPTABLES", "m")
    config("IP_NF_ARPFILTER", "m")
    config("IP_NF_ARP_MANGLE", "m")
}
addtask generate_config before do_configure after do_unpack

#KBUILD_DEFCONFIG:arm = "multi_v7_defconfig"

KBUILD_DEFCONFIG:aarch64 = "defconfig"
KBUILD_DEFCONFIG:arm:rpi = "bcm2835_defconfig"
COMPATIBLE_MACHINE:rpi = ".*"

KBUILD_DEFCONFIG:raspberrypi0-wifi ?= "bcm2835_defconfig"
COMPATIBLE_MACHINE:raspberrypi0-wifi = ".*"

python do_generate_config:append:rpi () {
    with open(d.expand("${B}/.config"), "a") as f:
        # make sure aarch64/arm matches in base defconfig
        f.write("CONFIG_CLK_RASPBERRYPI=y\n")
        f.write("CONFIG_ARM_RASPBERRYPI_CPUFREQ=y\n")
        f.write("CONFIG_SPI_BCM2835=y\n")
        f.write("CONFIG_SPI_BCM2835AUX=y\n")
        f.write("CONFIG_BCM2711_THERMAL=y\n")
        f.write("CONFIG_BCM2835_THERMAL=y\n")
        f.write("CONFIG_USB_LAN78XX=y\n")

        # framebuffer console
        f.write("CONFIG_DRM=y\n")
        f.write("CONFIG_DRM_VC4=y\n")
        f.write("CONFIG_FB_SIMPLE=y\n")
        f.write("CONFIG_LOGO=n\n")

        # pi3 - arm32 - neon support
        if "neon" in d.getVar("TUNE_FEATURES"):
            f.write("CONFIG_NEON=y\n")
            f.write("CONFIG_KERNEL_MODE_NEON=y\n")

        # camera
        f.write("CONFIG_STAGING=y\n")
        f.write("CONFIG_BCM_VIDEOCORE=y\n")
        f.write("CONFIG_BCM2835_VCHIQ=m\n")
        f.write("CONFIG_VIDEO_BCM2835=m\n")

        # hwrng
        f.write("CONFIG_HW_RANDOM=y\n")
        f.write("CONFIG_HW_RANDOM_BCM2835=y\n")
        f.write("CONFIG_HW_RANDOM_IPROC_RNG200=y\n")

        # of/dtb overlay support
        append(f, "DTC", "y")
        append(f, "OF", "y")
        append(f, "OF_OVERLAY", "y")

        append(f, "USB_CONFIGFS", "y")
        append(f, "USB_CONFIGFS_F_FS", "y")
        append(f, "USB_CONFIGFS_ACM", "y")
        append(f, "USB_CONFIGFS_NCM", "y")
        append(f, "USB_CONFIGFS_EEM", "y")
        append(f, "USB_CONFIGFS_F_UVC", "y")

        # usb cdc-ether (host side)
        append(f, "USB_USBNET", "y")
        append(f, "USB_NET_CDCETHER", "y")
        append(f, "USB_NET_CDC_EEM", "y")
        append(f, "USB_NET_CDC_NCM", "y")

        # needed for usb serial devices (just build them in)
        append(f, "USB_SERIAL", "y")
        append(f, "USB_SERIAL_GENERIC", "y")
        append(f, "USB_SERIAL_ARK3116", "y")
        append(f, "USB_SERIAL_BELKIN", "y")
        append(f, "USB_SERIAL_CH341", "y")
        append(f, "USB_SERIAL_CP210X", "y")
        append(f, "USB_SERIAL_FTDI_SIO", "y")
        append(f, "USB_SERIAL_F81232", "y")
        append(f, "USB_SERIAL_IPW", "y")
        append(f, "USB_SERIAL_PL2303", "y")
        append(f, "USB_SERIAL_OTI6858", "y")
        append(f, "USB_SERIAL_QUALCOMM", "y")
        append(f, "USB_SERIAL_SPCP8X5", "y")
        append(f, "USB_SERIAL_SIERRAWIRELESS", "y")
        append(f, "USB_SERIAL_TI", "y")
        append(f, "USB_SERIAL_WWAN", "y")
        append(f, "USB_SERIAL_OPTION", "y")
        append(f, "USB_ACM", "y")

    config("SPI_SPIDEV", "y")
}

python do_generate_config:append:qemuarm () {
    config("DEVTMPFS", "y")
    config("IKCONFIG_PROC", "y")

    config("USB_DUMMY_HCD", "y") # emulate a UDC
    config("USB_CONFIGFS", "y")
    config("USB_CONFIGFS_F_FS", "y")
    config("USB_CONFIGFS_ACM", "y")
    config("USB_CONFIGFS_NCM", "y")
    config("USB_CONFIGFS_EEM", "y")
    config("USB_CONFIGFS_F_UVC", "y")

    config("MEDIA_SUPPORT", "y")
    config("VIDEO_DEV", "y")
    config("VIDEO_V4L2", "y")
    config("USB_USB_F_UVC", "y")
}
KBUILD_DEFCONFIG:qemuarm = "multi_v7_defconfig"
COMPATIBLE_MACHINE:qemuarm = ".*"

python do_generate_config:append:toolbox-x64 () {
    with open(d.expand("${B}/.config"), "a") as f:
        f.write("CONFIG_EFI_STUB=y\n")
        # allow cpu to be trusted to provide entropy
        f.write("CONFIG_RANDOM_TRUST_CPU=y\n")
}
KBUILD_DEFCONFIG:toolbox-x64 = "x86_64_defconfig"
COMPATIBLE_MACHINE:toolbox-x64 = ".*"
