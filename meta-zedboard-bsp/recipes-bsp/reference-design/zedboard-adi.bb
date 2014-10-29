SUMMARY = "Reference Design Files"
DESCRIPTION = "Contains the Reference Design Files and pre-built bitstream."
HOMEPAGE = ""
SECTION = "base"

INHIBIT_DEFAULT_DEPS = "1"
PACKAGE_ARCH = "${MACHINE_ARCH}"

# The common license used for the Proprietary reference designs.
# Some reference designs have mixed GPL/LGPL parts as well, please refer to the
# individual reference design for more information on it's license.
LICENSE = "Proprietary"

LIC_FILES_CHKSUM = "file://system.bit;md5=9b34c7428eb5bd4c287017804dbbf2d2"

PROVIDES = "virtual/bitstream"

REPO_ROOT = "https://github.com/analogdevicesinc/fpgahdl_xilinx/raw/master/cf_adv7511_zed/SDK/SDK_Workspace/hw"
SRC_URI = " \
		${REPO_ROOT}/system.bit;name=bitstream \
		${REPO_ROOT}/ps7_init.h;name=ps7_inith \
		${REPO_ROOT}/ps7_init.c;name=ps7_initc \
		"
SRC_URI[bitstream.md5sum] = "9b34c7428eb5bd4c287017804dbbf2d2"
SRC_URI[bitstream.sha256sum] = "9f4cfcc9f12f4f5a108e79f8c01441c67dae983cd04107da62d1f6d9b2b63e04"
SRC_URI[ps7_inith.md5sum] = "afd9fd44611512c4ac5706bed9cf463f"
SRC_URI[ps7_inith.sha256sum] = "2b69fb13e686918ae288f5a22f91ae2e8a5e56d2a58d3aa297b1d2d98a93d370"
SRC_URI[ps7_initc.md5sum] = "7b507fd03935018307773ab78a0d0dc7"
SRC_URI[ps7_initc.sha256sum] = "2fe435da8d4c68049893dd65780ff1bebaaae23d3dce591fb418430f526ec78d"


S = "${WORKDIR}"

do_compile() {
	:
}

do_install() {
	if [ -e ${WORKDIR}/system.bit ]; then
		install -d ${D}/boot
		install ${WORKDIR}/system.bit ${D}/boot/system.bit
	fi
}

do_populate_sysroot() {
	for i in ps7_init.c ps7_init.h; do
		if [ -e ${WORKDIR}/$i ]; then
			install -d ${SYSROOT_DESTDIR}/boot/fsbl
			install ${WORKDIR}/$i ${SYSROOT_DESTDIR}/boot/fsbl/$i
		fi
	done
}

do_deploy() {
	if [ -e ${WORKDIR}/system.bit ]; then
		install -d ${DEPLOY_DIR_IMAGE}
		install ${WORKDIR}/system.bit ${DEPLOY_DIR_IMAGE}/system.bit
	fi
}

addtask deploy before do_build after do_install

FILES_${PN} += "/boot/system.bit"

COMPATIBLE_MACHINE = "zedboard-adi-zynq7"
