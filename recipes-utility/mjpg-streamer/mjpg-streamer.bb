SUMMARY = "Stream UVC, or similar as MJPG frames over HTTP"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://LICENSE;md5=751419260aa954499f7abaabaa882bbe"

SRC_URI = "git://github.com/jacksonliam/mjpg-streamer;protocol=https;branch=master"
SRCREV = "310b29f4a94c46652b20c4b7b6e5cf24e532af39"
PV = "1.0.0+git${SRCPV}"

S = "${WORKDIR}/git/mjpg-streamer-experimental"

DEPENDS += "jpeg"

inherit cmake

OECMAKE_GENERATOR = "Unix Makefiles"

PACKAGECONFIG ??= "uvc"
PACKAGECONFIG[http] = "-DENABLE_HTTP_MANAGEMENT=ON,-DENABLE_HTTP_MANAGEMENT=OFF"
PACKAGECONFIG[uvc] = "-DPLUGIN_INPUT_UVC=ON,-DPLUGIN_INPUT_UVC=OFF,v4l-utils"
PACKAGECONFIG[raspicam] = "-DPLUGIN_INPUT_RASPICAM=ON,-DPLUGIN_INPUT_RASPICAM=OFF,userland"

# Make it rpi specific due to depending on rpi binaries
# PACKAGECONFIG:append:rpi = "${@bb.utils.contains("MACHINE_FEATURES", "vc4graphics", " raspicam", "", d)}"
# PACKAGE_ARCH:rpi = "${MACHINE_ARCH}"
# needs to link with libmmal_vc_client.so
# ASNEEDED:rpi = ""

do_configure:prepend() {
    # HACK: replace include file check for raspi
    sed -i 's/HAS_RASPI OFF/HAS_RASPI ON/g' ${S}/plugins/input_raspicam/CMakeLists.txt
}

do_install() {
    oe_runmake install DESTDIR=${D}
}

# .so files are used as plugin libraries
FILES:${PN} += "${libdir}/*.so"

