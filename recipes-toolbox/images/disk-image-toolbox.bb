DESCRIPTION = "Disk image for the default toolbox bootable image."

PACKAGE_INSTALL = ""
IMAGE_INSTALL = ""
IMAGE_LINGUAS = ""

LICENSE = "MIT"

inherit core-image

python () {
    if "rpi" in d.getVar("OVERRIDES").split(":"):
        # for rpi, pack the initramfs into the boot partition
        d.appendVarFlag("do_image_wic", "depends", " bootfiles:do_deploy")
        d.appendVarFlag('do_rootfs', 'depends', ' core-image-toolbox:do_image_complete')
        d.appendVar("IMAGE_BOOT_FILES", " core-image-toolbox-${MACHINE}.cpio.gz;initramfs.gz")
        d.appendVar("IMAGE_BOOT_FILES", " bootfiles/*.txt")
        d.appendVar("IMAGE_BOOT_FILES", " bootfiles/*.bin")
        d.appendVar("IMAGE_BOOT_FILES", " bootfiles/*.dat")
        d.appendVar("IMAGE_BOOT_FILES", " bootfiles/*.elf")

    if "x86-64" in d.getVar("OVERRIDES").split(":"):
        # put kernel stubed efi image into a directory that works for removable disks
        d.appendVar("IMAGE_BOOT_FILES", " ${KERNEL_IMAGETYPE}-initramfs-${MACHINE}.bin;EFI/boot/bootx64.efi")
}

IMAGE_FSTYPES = "wic"
WKS_FILES = "vfat-bootonly.wks"

