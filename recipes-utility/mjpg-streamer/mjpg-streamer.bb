SUMMARY = "Stream UVC, or similar as MJPG frames over HTTP"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://LICENSE;md5=751419260aa954499f7abaabaa882bbe"

SRC_URI = "git://github.com/jacksonliam/mjpg-streamer.git;protocol=https"
SRCREV = "501f6362c5afddcfb41055f97ae484252c85c912"
PV = "0.4+git${SRCPV}"

S = "${WORKDIR}/git/mjpg-streamer-experimental"

DEPENDS += "jpeg"

inherit cmake

OECMAKE_GENERATOR = "Unix Makefiles"

PACKAGECONFIG ??= "http"
PACKAGECONFIG[http] = "-DENABLE_HTTP_MANAGEMENT=ON,-DENABLE_HTTP_MANAGEMENT=OFF"
PACKAGECONFIG[uvc] = "-DPLUGIN_INPUT_UVC=ON,-DPLUGIN_INPUT_UVC=OFF,v4l-utils"
PACKAGECONFIG[raspicam] = "-DPLUGIN_INPUT_RASPICAM=ON,-DPLUGIN_INPUT_RASPICAM=OFF,userland"

# Make it rpi specific due to depending on rpi binaries
PACKAGECONFIG_append_rpi = "${@bb.utils.contains("MACHINE_FEATURES", "vc4graphics", " raspicam", "", d)}"
PACKAGE_ARCH_rpi = "${MACHINE_ARCH}"
# needs to link with libmmal_vc_client.so
ASNEEDED_rpi = ""

do_configure_prepend() {
    # HACK: replace include file check for raspi
    sed -i 's/HAS_RASPI OFF/HAS_RASPI ON/g' ${S}/plugins/input_raspicam/CMakeLists.txt
}

do_install() {
    oe_runmake install DESTDIR=${D}
}

# .so files are used as plugin libraries
FILES_${PN} += "${libdir}/*.so"

