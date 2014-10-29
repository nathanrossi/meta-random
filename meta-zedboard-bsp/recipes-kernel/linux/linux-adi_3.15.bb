LINUX_VERSION = "3.15"
SRCREV ?= "09f7efe1efe4d6c714c4b2aae756a5bb6b90c3b4"
KBRANCH ?= "xcomm_zynq"

include recipes-kernel/linux/linux-xlnx.inc

SRC_URI = " \
		git://github.com/analogdevicesinc/linux.git;protocol=https;branch=${KBRANCH} \
		file://xilinx-base;type=kmeta;destsuffix=xilinx-base \
		"


FILESEXTRAPATHS_prepend := "${THISDIR}:"
SRC_URI_append += " \
		file://configs;type=kmeta;destsuffix=configs \
		"

