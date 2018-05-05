DESCRIPTION = "Rootfs image in initramfs format for the default toolbox distro."
LICENSE = "MIT"

IMAGE_LINGUAS = ""
IMAGE_INSTALL = " \
		packagegroup-core-boot \
		packagegroup-toolbox \
		kernel-modules \
		${CORE_IMAGE_EXTRA_INSTALL} \
		"

# always a cpio.gz, allow upto 256M
IMAGE_FSTYPES = "cpio.gz"
INITRAMFS_MAXSIZE = "268435456"

inherit core-image

