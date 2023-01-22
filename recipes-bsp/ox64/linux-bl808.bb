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

BRANCH = "linux-next/mboxic"
SRCREV = "5405206e44a950ad712eae685234b66446ec6393"
PV = "6.2-rc3+git${SRCPV}"
SRC_URI = "git://github.com/arm000/linux-bl808;protocol=https;branch=${BRANCH}"

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

KBUILD_DEFCONFIG:ox64 = "bl808_defconfig"
COMPATIBLE_MACHINE:ox64 = ".*"

python do_generate_config:append:ox64 () {
    # /proc/config
    config("IKCONFIG", "y")
    config("IKCONFIG_PROC", "y")

    # compress initramfs with zstd
    config("BLK_DEV_INITRD", "y")
    config("INITRAMFS_COMPRESSION_GZIP", "y")
    config("INITRAMFS_COMPRESSION_ZSTD", "y")
}

