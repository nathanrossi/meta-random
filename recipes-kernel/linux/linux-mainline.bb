SECTION = "kernel"
DESCRIPTION = "Mainline Linux kernel"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=d7810fab7487fb0aad327b76f1be7cd7"

inherit kernel
require recipes-kernel/linux/linux-dtb.inc

DEFAULT_PREFERENCE = "-1"

S = "${WORKDIR}/git"

BRANCH = "master"

SRCREV = "c8ae067f2635be0f8c7e5db1bb74b757d623e05b"
PV = "4.7-rc"
SRC_URI = "git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git;protocol=https;branch=${BRANCH}"

kernel_do_configure_prepend_zynq() {
	cp ${S}/arch/arm/configs/multi_v7_defconfig ${B}/.config

	echo "CONFIG_FPGA=y" >> ${B}/.config
	echo "CONFIG_FPGA_MGR_ZYNQ_FPGA=y" >> ${B}/.config
}

kernel_do_configure_prepend_zynqmp() {
	cp ${S}/arch/arm64/configs/defconfig ${B}/.config
}

KERNEL_DEVICETREE_zynq = "zynq-zybo.dtb zynq-zed.dtb zynq-zc702.dtb zynq-zc706.dtb"
KERNEL_DEVICETREE_ep108-zynqmp = "xilinx/zynqmp-ep108.dtb"

