LICENSE = "GPL-2.0+"
LIC_FILES_CHKSUM = "file://main.c;begineline=1;endline=1;md5=dcde81a56f7a67fc2c0d15658020f83a"

SRC_URI = "git://git.ideasonboard.org/uvc-gadget.git"
SRCREV = "105134f9a7a3faad9f02a6a5516a8cd8e625fb04"

SRC_URI += "file://patch.patch"

inherit cmake

#LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/GPL-2.0;md5=801f80980d171dd6425610833a22dbe6"
#SRC_URI = "git://github.com/wlhe/uvc-gadget;protocol=https"
#SRCREV = "743a050ee970d57e1cc4d8a4e0196a3c51ddc4b2"
#
#do_compile() {
#    oe_runmake CC="${CC}"
#}
#
#do_install() {
#    install -d ${D}${bindir}
#    install -m 0755 ${S}/uvc-gadget ${D}${bindir}/uvc-gadget
#}

RM_WORK_EXCLUDE += "${PN}"

S = "${WORKDIR}/git"

INSANE_SKIP_${PN} += "ldflags"
