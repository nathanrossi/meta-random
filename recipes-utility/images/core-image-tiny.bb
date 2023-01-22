SUMMARY = "A tiny image that contains only enough for a shell"

IMAGE_INSTALL = "base-files base-passwd busybox"

IMAGE_LINGUAS = " "

LICENSE = "MIT"

inherit core-image

IMAGE_ROOTFS_SIZE ?= "8192"
