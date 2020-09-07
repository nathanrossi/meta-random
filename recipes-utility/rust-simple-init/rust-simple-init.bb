SUMMARY = "Simple Rust Process Init System"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = "file://rust-simple-init.rs"
SRC_URI += "file://rtsp-restreamer.py"

S = "${WORKDIR}"
B = "${WORKDIR}/build"

inherit rust

do_configure[noexec] = "1"

# needed to point at target /usr/lib
export RUST_TARGET_PATH
# needed to get linker args to the called linker command
RUSTC_ARCHFLAGS += "-C link-args='${HOST_CC_ARCH} ${TOOLCHAIN_OPTIONS}'"

do_compile() {
    ${RUSTC} ${RUSTC_ARCHFLAGS} ${S}/rust-simple-init.rs
}

do_install() {
    install -d ${D}${base_sbindir} ${D}${bindir}
    install -m 0755 ${B}/rust-simple-init ${D}${base_sbindir}/init
    install -m 0755 ${S}/rtsp-restreamer.py ${D}${bindir}/rtsp-restreamer
}

FILES_${PN} += "${base_sbindir}/*"

# for rtsp-streamer
RDEPENDS_${PN} += "python3-core python3-io python3-netserver"

INSANE_SKIP_${PN} += "ldflags"
