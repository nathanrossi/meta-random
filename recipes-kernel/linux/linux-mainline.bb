SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://COPYING;md5=6bc538ed5bd9a7fc9398086aedcd7e46"

inherit kernel

# for ORC
DEPENDS += "elfutils-native"

# disable kernel-base depending on image, other mechanisms are used to ship the kernel
RDEPENDS:${KERNEL_PACKAGE_NAME}-base = ""
RRECOMMENDS:${KERNEL_PACKAGE_NAME}-base = ""

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "dd5a440a31fae6e459c0d6271dddd62825505361"
PV = "6.9-rc7+git${SRCPV}"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

# Build all kernel device trees with overlay/symbol support
KERNEL_DTC_FLAGS = "-@"

python do_generate_config:append() {
    config("DEVTMPFS", "y")
    config("TMPFS", "y")

    # /proc/config
    config("IKCONFIG", "y")
    config("IKCONFIG_PROC", "y")

    # compress kernel with zstd
    config("KERNEL_ZSTD", "y")

    # enable kexec
    config("KEXEC", "y")
    config("KEXEC_FILE", "y")
    config("KEXEC_JUMP", "y")

    # RTC
    config("RTC_INTF_SYSFS", "y")
    config("RTC_INTF_PROC", "y")
    config("RTC_INTF_DEV", "y")

    config("DRM", "n")
    config("SOUND", "n")
    config("SND", "n")
    config("VIRTIO", "n")
    config("MISC_FILESYSTEMS", "n")
    config("NETWORK_FILESYSTEMS", "n")
    config("CRYPTO", "n")
    config("FTRACE", "n")
    config("BPF", "n")
    config("NF_CONNTRACK", "n")
    config("NETFILTER_XTABLES", "n")
    config("MAC80211", "n")
    config("NET_9P", "n")
    config("YENTA", "n")
    config("MD", "n")
    config("WLAN", "n")
    config("INPUT", "n")
    config("CC_OPTIMIZE_FOR_SIZE", "y")

    # iptables/etc modules
    # config("IP_NF_IPTABLES", "m")
    # config("IP_NF_MATCH_ECN", "m")
    # config("IP_NF_MATCH_TTL", "m")
    # config("IP_NF_FILTER", "m")
    # config("IP_NF_TARGET_REJECT", "m")
    # config("IP_NF_TARGET_MASQUERADE", "m")
    # config("IP_NF_TARGET_REDIRECT", "m")
    # config("IP_NF_TARGET_NETMAP", "m")
    # config("IP_NF_MANGLE", "m")
    # config("IP_NF_TARGET_ECN", "m")
    # config("IP_NF_TARGET_CLUSTERIP", "m")
    # config("IP_NF_RAW", "m")
    # config("IP_NF_ARPTABLES", "m")
    # config("IP_NF_ARPFILTER", "m")
    # config("IP_NF_ARP_MANGLE", "m")
}
addtask generate_config before do_configure after do_unpack

#KBUILD_DEFCONFIG:arm = "multi_v7_defconfig"

KBUILD_DEFCONFIG:aarch64 = "defconfig"
KBUILD_DEFCONFIG:arm:rpi = "bcm2835_defconfig"
COMPATIBLE_MACHINE:rpi = ".*"

KBUILD_DEFCONFIG:raspberrypi0-wifi ?= "bcm2835_defconfig"
COMPATIBLE_MACHINE:raspberrypi0-wifi = ".*"

python do_generate_config:append:rpi () {
    # make sure aarch64/arm matches in base defconfig
    config("CLK_RASPBERRYPI", "y")
    config("ARM_RASPBERRYPI_CPUFREQ", "y")
    config("SPI_BCM2835", "y")
    config("SPI_BCM2835AUX", "y")
    config("BCM2711_THERMAL", "y")
    config("BCM2835_THERMAL", "y")

    # framebuffer console
    config("DRM", "y")
    config("DRM_VC4", "y")
    config("FB_SIMPLE", "y")
    config("LOGO", "n")

    # pi3 - arm32 - neon support
    if "neon" in d.getVar("TUNE_FEATURES"):
        config("NEON", "y")
        config("KERNEL_MODE_NEON", "y")

    # USB ethernet
    config("USB_LAN78XX", "y")
    # pi4 ethernet
    config("BCMGENET", "y")

    # pi4 pcie
    config("PCI", "y")
    config("PCIE_BRCMSTB", "y")

    # camera
    config("STAGING", "y")
    config("BCM_VIDEOCORE", "y")
    config("BCM2835_VCHIQ", "m")
    config("VIDEO_BCM2835", "m")

    # hwrng
    config("HW_RANDOM", "y")
    config("HW_RANDOM_BCM2835", "y")
    config("HW_RANDOM_IPROC_RNG200", "y")

    # of/dtb overlay support
    config("DTC", "y")
    config("OF", "y")
    config("OF_OVERLAY", "y")

    config("USB_CONFIGFS", "y")
    config("USB_CONFIGFS_F_FS", "y")
    config("USB_CONFIGFS_ACM", "y")
    config("USB_CONFIGFS_NCM", "y")
    config("USB_CONFIGFS_EEM", "y")
    config("USB_CONFIGFS_F_UVC", "y")

    # usb cdc-ether (host side)
    config("USB_USBNET", "y")
    config("USB_NET_CDCETHER", "y")
    config("USB_NET_CDC_EEM", "y")
    config("USB_NET_CDC_NCM", "y")

    # needed for usb serial devices (just build them in)
    config("USB_SERIAL", "y")
    config("USB_SERIAL_GENERIC", "y")
    config("USB_SERIAL_ARK3116", "y")
    config("USB_SERIAL_BELKIN", "y")
    config("USB_SERIAL_CH341", "y")
    config("USB_SERIAL_CP210X", "y")
    config("USB_SERIAL_FTDI_SIO", "y")
    config("USB_SERIAL_F81232", "y")
    config("USB_SERIAL_IPW", "y")
    config("USB_SERIAL_PL2303", "y")
    config("USB_SERIAL_OTI6858", "y")
    config("USB_SERIAL_QUALCOMM", "y")
    config("USB_SERIAL_SPCP8X5", "y")
    config("USB_SERIAL_SIERRAWIRELESS", "y")
    config("USB_SERIAL_TI", "y")
    config("USB_SERIAL_WWAN", "y")
    config("USB_SERIAL_OPTION", "y")
    config("USB_ACM", "y")

    config("SPI_SPIDEV", "y")
}

python do_generate_config:append:qemuarm () {
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

python do_generate_config:append:genericx86-64 () {
    # force logging to serial console
    config("CMDLINE_BOOL", "y")
    config("CMDLINE", "\"console=ttyS0,115200\"")

    # tty0/framebuffer
    # config("FB", "y")
    # config("FRAMEBUFFER_CONSOLE", "y")
    # config("FB_EFI", "y")

    # networking (IGC/i225/i226)
    config("IGC", "y")
    # >=10G networking
    config("MLX4_EN", "y")
    config("IXGBE", "y")
    config("ICE", "y")

    # Sensors
    config("INTEL_PCH_THERMAL", "y")
    config("SENSORS_IT87", "y")
    config("SENSORS_NCT6683", "y")
    config("SENSORS_NCT6775_CORE", "y")
    config("SENSORS_NCT6775", "y")
    config("SENSORS_NCT7802", "y")
    config("SENSORS_NCT7904", "y")
    config("SENSORS_ASUS_WMI", "y")
    config("SENSORS_ASUS_EC", "y")

    # USB
    # CONFIG_USB_XHCI_PLATFORM=y
    # CONFIG_USB_PCI=y
    # CONFIG_USB_DEFAULT_PERSIST=y
    # CONFIG_USB_XHCI_PCI=y
    # CONFIG_USB_EHCI_TT_NEWSCHED=y
    # CONFIG_USB_EHCI_PCI=y
    # CONFIG_USB_OHCI_HCD_PCI=y
    # CONFIG_USB_UHCI_HCD=y

    # thermal/temperature
    config("SENSORS_CORETEMP", "y")
}
KBUILD_DEFCONFIG:genericx86-64 = "x86_64_defconfig"
COMPATIBLE_MACHINE:genericx86-64 = ".*"
