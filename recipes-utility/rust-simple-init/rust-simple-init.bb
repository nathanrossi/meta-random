SUMMARY = "Simple Rust Process Init System"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

SRC_URI = " \
    file://src \
    file://Cargo.toml \
    "

# crate dependencies
SRC_URI += " \
    crate://crates.io/arc-swap/0.4.7 \
    crate://crates.io/bitflags/1.2.1 \
    crate://crates.io/bytes/0.5.6 \
    crate://crates.io/cc/1.0.60 \
    crate://crates.io/cfg-if/0.1.10 \
    crate://crates.io/fuchsia-zircon-sys/0.3.3 \
    crate://crates.io/fuchsia-zircon/0.3.3 \
    crate://crates.io/futures-channel/0.3.5 \
    crate://crates.io/futures-core/0.3.5 \
    crate://crates.io/futures-executor/0.3.5 \
    crate://crates.io/futures-io/0.3.5 \
    crate://crates.io/futures-macro/0.3.5 \
    crate://crates.io/futures-sink/0.3.5 \
    crate://crates.io/futures-task/0.3.5 \
    crate://crates.io/futures-util/0.3.5 \
    crate://crates.io/futures/0.3.5 \
    crate://crates.io/iovec/0.1.4 \
    crate://crates.io/kernel32-sys/0.2.2 \
    crate://crates.io/lazy_static/1.4.0 \
    crate://crates.io/libc/0.2.78 \
    crate://crates.io/log/0.4.11 \
    crate://crates.io/memchr/2.3.3 \
    crate://crates.io/mio-named-pipes/0.1.7 \
    crate://crates.io/mio-uds/0.6.8 \
    crate://crates.io/mio/0.6.22 \
    crate://crates.io/miow/0.2.1 \
    crate://crates.io/miow/0.3.5 \
    crate://crates.io/net2/0.2.35 \
    crate://crates.io/once_cell/1.4.1 \
    crate://crates.io/pin-project-internal/0.4.23 \
    crate://crates.io/pin-project-lite/0.1.7 \
    crate://crates.io/pin-project/0.4.23 \
    crate://crates.io/pin-utils/0.1.0 \
    crate://crates.io/proc-macro-hack/0.5.18 \
    crate://crates.io/proc-macro-nested/0.1.6 \
    crate://crates.io/proc-macro2/1.0.21 \
    crate://crates.io/quote/1.0.7 \
    crate://crates.io/redox_syscall/0.1.57 \
    crate://crates.io/signal-hook-registry/1.2.1 \
    crate://crates.io/slab/0.4.2 \
    crate://crates.io/socket2/0.3.15 \
    crate://crates.io/syn/1.0.41 \
    crate://crates.io/tokio-macros/0.2.5 \
    crate://crates.io/tokio/0.2.22 \
    crate://crates.io/unicode-xid/0.2.1 \
    crate://crates.io/winapi-build/0.1.1 \
    crate://crates.io/winapi-i686-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi-x86_64-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi/0.2.8 \
    crate://crates.io/winapi/0.3.9 \
    crate://crates.io/ws2_32-sys/0.2.1 \
    git://github.com/nix-rust/nix;protocol=https;nobranch=1;name=nix;destsuffix=nix \
    "
SRCREV_nix = "${AUTOREV}"
EXTRA_OECARGO_PATHS += "${WORKDIR}/nix"

SRC_URI += "file://rtsp-restreamer.py"

S = "${WORKDIR}"
B = "${WORKDIR}/build"

inherit rust
inherit cargo

do_install_append() {
    install -d ${D}${base_sbindir} ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/rust-simple-init ${D}${base_sbindir}/init
    # install -m 0755 ${B}/rust-simple-init ${D}${base_sbindir}/init
    install -m 0755 ${S}/rtsp-restreamer.py ${D}${bindir}/rtsp-restreamer
}

FILES_${PN} += "${base_sbindir}/*"

# for rtsp-streamer
RDEPENDS_${PN} += "python3-core python3-io python3-netserver"

INSANE_SKIP_${PN} += "ldflags"

INHIBIT_PACKAGE_STRIP = "1"
# INHIBIT_PACKAGE_STRIP_FILES = "${base_sbindir}/init"

RDEPENDS_${PN} += "uvc-gadget"
