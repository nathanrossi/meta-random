SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=6bc538ed5bd9a7fc9398086aedcd7e46"

inherit kernel

# disable kernel-base depending on image, other mechanisms are used to ship the kernel
RDEPENDS:${KERNEL_PACKAGE_NAME}-base = ""

DEFAULT_PREFERENCE = "-1"
COMPATIBLE_MACHINE = "^$"

S = "${WORKDIR}/git"

BRANCH = "master"
SRCREV = "f8e6dfc64f6135d1b6c5215c14cd30b9b60a0008"
PV = "5.14-rc5+git${SRCPV}"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

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

    # other networking features for routing
    config("IFB", "m")
    config("NET_CLS_FW", "m")
    config("CLS_U32_MARK", "m")
    config("NET_ACT_CONNMARK", "m")
    config("NET_SCH_CAKE", "m")
    config("NET_SCH_INGRESS", "m")
    config("NET_SCH_FIFO", "m")
    config("NET_SCH_PRIO", "m")
    config("NET_SCH_NETEM", "m")

    config("VETH", "y")
    config("BRIDGE", "y")
    config("BRIDGE_VLAN_FILTERING", "y")
}
