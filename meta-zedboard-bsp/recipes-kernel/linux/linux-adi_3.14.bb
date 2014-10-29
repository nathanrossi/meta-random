LINUX_VERSION = "3.14"
SRCREV ?= "3a5cdbdaded2e5b577b3fc64e8e0606509777daf"
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

