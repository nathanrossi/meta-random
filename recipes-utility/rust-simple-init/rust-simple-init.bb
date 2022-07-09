SUMMARY = "Simple Rust Process Init System"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = " \
    file://src \
    file://Cargo.toml \
    "

# crate dependencies
SRC_URI += " \
    crate://crates.io/autocfg/1.1.0 \
    crate://crates.io/bitflags/1.3.2 \
    crate://crates.io/cfg-if/0.1.10 \
    crate://crates.io/cfg-if/1.0.0 \
    crate://crates.io/fuchsia-zircon-sys/0.3.3 \
    crate://crates.io/fuchsia-zircon/0.3.3 \
    crate://crates.io/futures-channel/0.3.21 \
    crate://crates.io/futures-core/0.3.21 \
    crate://crates.io/futures-executor/0.3.21 \
    crate://crates.io/futures-io/0.3.21 \
    crate://crates.io/futures-macro/0.3.21 \
    crate://crates.io/futures-sink/0.3.21 \
    crate://crates.io/futures-task/0.3.21 \
    crate://crates.io/futures-util/0.3.21 \
    crate://crates.io/futures/0.3.21 \
    crate://crates.io/iovec/0.1.4 \
    crate://crates.io/kernel32-sys/0.2.2 \
    crate://crates.io/libc/0.2.126 \
    crate://crates.io/log/0.4.17 \
    crate://crates.io/memchr/2.5.0 \
    crate://crates.io/memoffset/0.6.5 \
    crate://crates.io/mio/0.6.23 \
    crate://crates.io/miow/0.2.2 \
    crate://crates.io/net2/0.2.37 \
    crate://crates.io/nix/0.24.1 \
    crate://crates.io/pin-project-lite/0.2.9 \
    crate://crates.io/pin-utils/0.1.0 \
    crate://crates.io/proc-macro2/1.0.40 \
    crate://crates.io/quote/1.0.20 \
    crate://crates.io/slab/0.4.6 \
    crate://crates.io/syn/1.0.98 \
    crate://crates.io/unicode-ident/1.0.1 \
    crate://crates.io/winapi-build/0.1.1 \
    crate://crates.io/winapi-i686-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi-x86_64-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi/0.2.8 \
    crate://crates.io/winapi/0.3.9 \
    crate://crates.io/ws2_32-sys/0.2.1 \
    "

S = "${WORKDIR}"
B = "${WORKDIR}/build"

inherit rust
inherit cargo

do_install:append() {
    install -d ${D}${base_sbindir} ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/rust-simple-init ${D}${base_sbindir}/init
}

FILES:${PN} += "${base_sbindir}/*"

INSANE_SKIP:${PN} += "ldflags"

INHIBIT_PACKAGE_STRIP = "1"

RDEPENDS:${PN} += "wpa-supplicant"
RDEPENDS:${PN} += "mjpg-streamer"
