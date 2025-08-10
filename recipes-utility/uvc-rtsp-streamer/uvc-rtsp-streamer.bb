SUMMARY = "UVC RTSP Streamer"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

inherit externalsrc
EXTERNALSRC = "/home/nathan/dev/uvc-rtsp-streamer"

inherit rust
inherit cargo
inherit pkgconfig

# for bindgen
DEPENDS += "clang-native"
export BINDGEN_EXTRA_CLANG_ARGS = "--sysroot=${RECIPE_SYSROOT}"

DEPENDS += "v4l-utils"
DEPENDS += "alsa-lib"
DEPENDS += "libcamera"

B = "${WORKDIR}/build"

inherit cargo-update-recipe-crates
require ${BPN}-crates.inc

do_install:append() {
    install -d ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/uvc-rtsp-streamer ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/capture ${D}${bindir}/uvc-rtsp-capture
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/rtsp-loopback ${D}${bindir}/uvc-rtsp-loopback
}

