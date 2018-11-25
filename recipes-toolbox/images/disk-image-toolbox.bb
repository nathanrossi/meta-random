DESCRIPTION = "Disk image for the default toolbox bootable image."

PACKAGE_INSTALL = ""
IMAGE_INSTALL = ""
IMAGE_LINGUAS = ""

LICENSE = "MIT"

inherit core-image

python () {
    if "rpi" in d.getVar("MACHINEOVERRIDES").split(":"):
        # for rpi, pack the initramfs into the boot partition
        d.appendVarFlag("do_image_wic", "depends", " bcm2835-bootfiles:do_deploy")
        d.appendVarFlag('do_rootfs', 'depends', ' core-image-toolbox:do_image_complete')
        d.appendVar("IMAGE_BOOT_FILES", " core-image-toolbox-${MACHINE}.cpio.gz;initramfs.gz")
        d.appendVar("IMAGE_BOOT_FILES", " bcm2835-bootfiles/*.txt")
        d.appendVar("IMAGE_BOOT_FILES", " bcm2835-bootfiles/*.bin")
        d.appendVar("IMAGE_BOOT_FILES", " bcm2835-bootfiles/*.dat")
        d.appendVar("IMAGE_BOOT_FILES", " bcm2835-bootfiles/*.elf")
}

IMAGE_FSTYPES = "wic"
WKS_FILES = "vfat-bootonly.wks"

