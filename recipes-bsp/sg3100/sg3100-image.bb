DESCRIPTION = "Bootable kernel image for SG3100"
LICENSE = "MIT"

inherit deploy

COMPATIBLE_MACHINE = "sg3100"
PACKAGE_ARCH = "${MACHINE_ARCH}"

do_configure[noexec] = "1"
do_install[noexec] = "1"

do_compile[depends] += "virtual/kernel:do_deploy"
do_compile[depends] += "sg3100-devicetree:do_deploy"

do_compile() {
    cat \
        ${DEPLOY_DIR_IMAGE}/zImage-initramfs-${MACHINE}.bin \
        ${DEPLOY_DIR_IMAGE}/devicetree/armada-380-sg3100.dtb \
        > ${B}/zImage-bundle-${MACHINE}.bin
}

do_deploy () {
    install -Dm 0644 ${B}/zImage-bundle-${MACHINE}.bin ${DEPLOYDIR}/zImage-bundle-${MACHINE}.bin
}
addtask deploy before do_build after do_install

# Factory U-Boot does not boot the kernel with device tree machid, and does not
# inject initramfs address into device tree.
#
# U-Boot commands to boot:
# dhcp; set serverip 10.0.0.164; tftpboot 0x02000000 zImage-bundle-sg3100.bin; bootz 0x02000000

