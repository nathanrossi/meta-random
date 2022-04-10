SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://COPYING;md5=6bc538ed5bd9a7fc9398086aedcd7e46"

inherit kernel

# disable kernel-base depending on image, other mechanisms are used to ship the kernel
RDEPENDS:${KERNEL_PACKAGE_NAME}-base = ""
RRECOMMENDS:${KERNEL_PACKAGE_NAME}-base = ""

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "1862a69c917417142190bc18c8ce16680598664b"
PV = "5.18-rc2+git${SRCPV}"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

python do_generate_config() {
    def config(name, val):
        cfgname = "CONFIG_{}".format(name)
        cfgcomment = "# " + cfgname
        data = ""
        with open(d.expand("${B}/.config"), "r") as source:
            matched = False
            for l in source:
                if l.startswith(cfgname + "=") or l.startswith(cfgcomment + " "):
                    data += "CONFIG_{}={}\n".format(name, val)
                    matched = True
                else:
                    data += l
            if not matched: # append if not set
                data += "CONFIG_{}={}\n".format(name, val)

        with open(d.expand("${B}/.config"), "w") as dest:
            dest.write(data)

    with open(d.expand("${B}/.config"), "w") as f:
        with open(d.expand("${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG}"), "r") as src:
            f.write(src.read())
}
addtask generate_config before do_configure after do_unpack

KBUILD_DEFCONFIG:sg3100 = "multi_v7_defconfig"
COMPATIBLE_MACHINE:sg3100 = ".*"

# hacks for switch chip
SRC_URI:append:sg3100 = " file://0001-WIP.patch "

python do_generate_config:append:sg3100 () {
    # /proc/config
    config("IKCONFIG", "y")
    config("IKCONFIG_PROC", "y")

    # compress initramfs with zstd
    config("INITRAMFS_COMPRESSION_ZSTD", "y")

    # uart early debug
    config("DEBUG_LL", "y")
    config("DEBUG_MVEBU_UART0_ALTERNATE", "y")

    # spi flash
    # HACK: Disable spi flash driver, as it does something to the spi flash
    # configuration that prevent reboot from working correctly.
    config("MTD_SPI_NOR", "n")

    # rtc
    config("RTC_CLASS", "y")
    config("RTC_DRV_ARMADA38X", "y")

    # leds
    config("LEDS_IS31FL319X", "y")

    # sata/ahci
    config("AHCI_MVEBU", "y")

    # DSA
    config("NET_DSA", "y")
    config("NET_DSA_TAG_DSA", "y")
    config("NET_DSA_TAG_EDSA", "y")
    config("NET_DSA_MV88E6XXX", "y")
    config("NET_DSA_MV88E6XXX_GLOBAL2", "y")
    config("NET_SWITCHDEV", "y")
    config("MARVELL_PHY", "y")
}
