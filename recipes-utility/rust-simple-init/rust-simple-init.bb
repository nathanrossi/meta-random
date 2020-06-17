SUMMARY = "Simple Rust Process Init System"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = "file://rust-simple-init.rs"

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
    install -d ${D}${base_sbindir}
    install -m 0755 ${B}/rust-simple-init ${D}${base_sbindir}/init
}

FILES_${PN} += "${base_sbindir}/*"

