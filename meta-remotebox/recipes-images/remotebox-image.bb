SUMMARY = "Image for a remotebox"

IMAGE_INSTALL = " \
		packagegroup-core-boot \
		${ROOTFS_PKGMANAGE_BOOTSTRAP} \
		${CORE_IMAGE_EXTRA_INSTALL} \
		packagegroup-remotebox-core \
		networkd-config \
		"

IMAGE_LINGUAS = " "

LICENSE = "MIT"

inherit core-image

