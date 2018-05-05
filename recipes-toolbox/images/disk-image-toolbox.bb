DESCRIPTION = "Disk image for the default toolbox bootable image."

PACKAGE_INSTALL = ""
IMAGE_INSTALL = ""
IMAGE_LINGUAS = ""

LICENSE = "MIT"

inherit core-image

python () {
    d.appendVarFlag('do_rootfs', 'depends', ' core-image-toolbox:do_image_complete')
}

IMAGE_FSTYPES = "wic"
WKS_FILES = "sdimage-bootonly.wks"

IMAGE_BOOT_FILES_append = " core-image-toolbox-${MACHINE}.cpio.gz;initramfs.gz"

