DESCRIPTION = "Boot script for SG3100"
LICENSE = "MIT"

inherit deploy

COMPATIBLE_MACHINE = "sg3100"
PACKAGE_ARCH = "${MACHINE_ARCH}"

DEPENDS += "u-boot-tools-native"

do_configure[noexec] = "1"
do_install[noexec] = "1"

do_compile() {
    # generate u-boot script file for distro bootcmd
    echo 'fatload ${devtype} ${devnum}:${distro_bootpart} ${kernel_addr_r} zImage' > ${B}/boot.cmd
    echo 'fatload ${devtype} ${devnum}:${distro_bootpart} ${ramdisk_addr_r} core-image-minimal-sg3100.cpio.gz.u-boot' >> ${B}/boot.cmd
    echo 'bootz ${kernel_addr_r} ${ramdisk_addr_r} ${fdtcontroladdr}' >> ${B}/boot.cmd
    mkimage -A arm -O linux -T script -C none -a 0 -e 0 -n "Boot Script" -d ${B}/boot.cmd ${B}/boot.scr
}

do_deploy () {
    install -Dm 0644 ${B}/boot.scr ${DEPLOYDIR}/boot.scr
}
addtask deploy before do_build after do_install
