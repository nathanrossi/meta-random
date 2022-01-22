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
    import contextlib
    @contextlib.contextmanager
    def config():
        configs = {}
        def enable(option):
            configs[option] = "y"
        def disable(option):
            configs[option] = "n"
        def module(option):
            configs[option] = "m"

        yield enable, disable, module

        data = ""
        pending = list(configs.keys())
        with open(d.expand("${B}/.config"), "r") as source:
            for l in source:
                if len(l) == 0:
                    data += l
                    continue

                matched = False
                for k, v in configs.items():
                    cfgname = "CONFIG_{}".format(k)
                    cfgcomment = "# " + cfgname
                    if l.startswith(cfgname + "=") or l.startswith(cfgcomment + " "):
                        data += "{}={}\n".format(cfgname, v)
                        matched = True
                        if k in pending:
                            pending.remove(k)
                        break
                if not matched: # append if not overriden
                    data += l

            for k in pending:
                data += "CONFIG_{}={}\n".format(k, configs.get(k))

        with open(d.expand("${B}/.config"), "w") as dest:
            dest.write(data)
}

python do_generate_config:append() {
    with config() as (enable, disable, module):
        # iptables/etc modules
        module("IP_NF_IPTABLES")
        module("IP_NF_MATCH_ECN")
        module("IP_NF_MATCH_TTL")
        module("IP_NF_FILTER")
        module("IP_NF_TARGET_REJECT")
        module("IP_NF_TARGET_MASQUERADE")
        module("IP_NF_TARGET_REDIRECT")
        module("IP_NF_TARGET_NETMAP")
        module("IP_NF_MANGLE")
        module("IP_NF_TARGET_ECN")
        module("IP_NF_TARGET_CLUSTERIP")
        module("IP_NF_RAW")
        module("IP_NF_ARPTABLES")
        module("IP_NF_ARPFILTER")
        module("IP_NF_ARP_MANGLE")
}
addtask generate_config before do_configure after do_unpack

#KBUILD_DEFCONFIG:arm = "multi_v7_defconfig"

KBUILD_DEFCONFIG:aarch64 = "defconfig"
KBUILD_DEFCONFIG:arm:rpi = "bcm2835_defconfig"
COMPATIBLE_MACHINE:rpi = ".*"

KBUILD_DEFCONFIG:raspberrypi0-wifi ?= "bcm2835_defconfig"
COMPATIBLE_MACHINE:raspberrypi0-wifi = ".*"

python do_generate_config:append:rpi () {
    with config() as (enable, disable, module):
        # make sure aarch64/arm matches in base defconfig
        enable("CLK_RASPBERRYPI")
        enable("ARM_RASPBERRYPI_CPUFREQ")
        enable("SPI_BCM2835")
        enable("SPI_BCM2835AUX")
        enable("BCM2711_THERMAL")
        enable("BCM2835_THERMAL")

        # framebuffer console
        enable("DRM")
        enable("DRM_VC4")
        enable("FB_SIMPLE")
        disable("LOGO")

        # pi3 - arm32 - neon support
        if "neon" in d.getVar("TUNE_FEATURES"):
            enable("NEON")
            enable("KERNEL_MODE_NEON")

        # USB ethernet
        enable("USB_LAN78XX")
        # pi4 ethernet
        enable("BCMGENET")

        # pi4 pcie
        enable("PCI")
        enable("PCIE_BRCMSTB")

        # camera
        enable("STAGING")
        enable("BCM_VIDEOCORE")
        module("BCM2835_VCHIQ")
        module("VIDEO_BCM2835")

        # hwrng
        enable("HW_RANDOM")
        enable("HW_RANDOM_BCM2835")
        enable("HW_RANDOM_IPROC_RNG200")

        # of/dtb overlay support
        enable("DTC")
        enable("OF")
        enable("OF_OVERLAY")

        enable("USB_CONFIGFS")
        enable("USB_CONFIGFS_F_FS")
        enable("USB_CONFIGFS_ACM")
        enable("USB_CONFIGFS_NCM")
        enable("USB_CONFIGFS_EEM")
        enable("USB_CONFIGFS_F_UVC")

        # usb cdc-ether (host side)
        enable("USB_USBNET")
        enable("USB_NET_CDCETHER")
        enable("USB_NET_CDC_EEM")
        enable("USB_NET_CDC_NCM")

        # needed for usb serial devices (just build them in)
        enable("USB_SERIAL")
        enable("USB_SERIAL_GENERIC")
        enable("USB_SERIAL_ARK3116")
        enable("USB_SERIAL_BELKIN")
        enable("USB_SERIAL_CH341")
        enable("USB_SERIAL_CP210X")
        enable("USB_SERIAL_FTDI_SIO")
        enable("USB_SERIAL_F81232")
        enable("USB_SERIAL_IPW")
        enable("USB_SERIAL_PL2303")
        enable("USB_SERIAL_OTI6858")
        enable("USB_SERIAL_QUALCOMM")
        enable("USB_SERIAL_SPCP8X5")
        enable("USB_SERIAL_SIERRAWIRELESS")
        enable("USB_SERIAL_TI")
        enable("USB_SERIAL_WWAN")
        enable("USB_SERIAL_OPTION")
        enable("USB_ACM")

        enable("SPI_SPIDEV")
}

python do_generate_config:append:qemuarm () {
    with config() as (enable, disable, module):
        enable("DEVTMPFS")
        enable("IKCONFIG_PROC")

        enable("USB_DUMMY_HCD") # emulate a UDC
        enable("USB_CONFIGFS")
        enable("USB_CONFIGFS_F_FS")
        enable("USB_CONFIGFS_ACM")
        enable("USB_CONFIGFS_NCM")
        enable("USB_CONFIGFS_EEM")
        enable("USB_CONFIGFS_F_UVC")

        enable("MEDIA_SUPPORT")
        enable("VIDEO_DEV")
        enable("VIDEO_V4L2")
        enable("USB_USB_F_UVC")
}
KBUILD_DEFCONFIG:qemuarm = "multi_v7_defconfig"
COMPATIBLE_MACHINE:qemuarm = ".*"

python do_generate_config:append:toolbox-x64 () {
    with config() as (enable, disable, module):
        enable("EFI_STUB")
        # allow cpu to be trusted to provide entropy
        enable("RANDOM_TRUST_CPU")
}
KBUILD_DEFCONFIG:toolbox-x64 = "x86_64_defconfig"
COMPATIBLE_MACHINE:toolbox-x64 = ".*"
