DESCRIPTION = "Disk image for core-image-minimal (used to access deployed core-image-minimal)"

PACKAGE_INSTALL = ""
IMAGE_INSTALL = ""
IMAGE_LINGUAS = ""

LICENSE = "MIT"

inherit core-image

IMAGE_FSTYPES = "wic"
WKS_FILES = "vfat-bootonly.wks"

do_image_wic[depends] += "core-image-minimal:do_image_complete"

