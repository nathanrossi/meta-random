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

# DEBUG_BUILD = "1"

# Skip checksums on crates
BB_STRICT_CHECKSUM = "0"

# crate dependencies
SRC_URI += " \
    crate://crates.io/aho-corasick/1.1.2 \
    crate://crates.io/alsa-sys/0.3.1 \
    crate://crates.io/alsa/0.8.1 \
    crate://crates.io/anyhow/1.0.80 \
    crate://crates.io/async-attributes/1.1.2 \
    crate://crates.io/async-channel/1.9.0 \
    crate://crates.io/async-channel/2.2.0 \
    crate://crates.io/async-executor/1.8.0 \
    crate://crates.io/async-global-executor/2.4.1 \
    crate://crates.io/async-io/1.13.0 \
    crate://crates.io/async-io/2.3.2 \
    crate://crates.io/async-listen/0.2.1 \
    crate://crates.io/async-lock/2.8.0 \
    crate://crates.io/async-lock/3.3.0 \
    crate://crates.io/async-process/1.8.1 \
    crate://crates.io/async-signal/0.2.5 \
    crate://crates.io/async-std/1.12.0 \
    crate://crates.io/async-stream-impl/0.3.5 \
    crate://crates.io/async-stream/0.3.5 \
    crate://crates.io/async-task/4.7.0 \
    crate://crates.io/atomic-waker/1.1.2 \
    crate://crates.io/autocfg/1.1.0 \
    crate://crates.io/bindgen/0.66.1 \
    crate://crates.io/bindgen/0.69.4 \
    crate://crates.io/bitflags/1.3.2 \
    crate://crates.io/bitflags/2.4.2 \
    crate://crates.io/blocking/1.5.1 \
    crate://crates.io/bumpalo/3.15.4 \
    crate://crates.io/cc/1.0.90 \
    crate://crates.io/cexpr/0.6.0 \
    crate://crates.io/cfg-if/1.0.0 \
    crate://crates.io/clang-sys/1.7.0 \
    crate://crates.io/concurrent-queue/2.4.0 \
    crate://crates.io/crossbeam-utils/0.8.19 \
    crate://crates.io/drm-fourcc/2.2.0 \
    crate://crates.io/either/1.10.0 \
    crate://crates.io/equivalent/1.0.1 \
    crate://crates.io/errno/0.3.8 \
    crate://crates.io/event-listener-strategy/0.4.0 \
    crate://crates.io/event-listener-strategy/0.5.0 \
    crate://crates.io/event-listener/2.5.3 \
    crate://crates.io/event-listener/3.1.0 \
    crate://crates.io/event-listener/4.0.3 \
    crate://crates.io/event-listener/5.2.0 \
    crate://crates.io/fastrand/1.9.0 \
    crate://crates.io/fastrand/2.0.1 \
    crate://crates.io/futures-channel/0.3.30 \
    crate://crates.io/futures-core/0.3.30 \
    crate://crates.io/futures-executor/0.3.30 \
    crate://crates.io/futures-io/0.3.30 \
    crate://crates.io/futures-lite/1.13.0 \
    crate://crates.io/futures-lite/2.2.0 \
    crate://crates.io/futures-macro/0.3.30 \
    crate://crates.io/futures-sink/0.3.30 \
    crate://crates.io/futures-task/0.3.30 \
    crate://crates.io/futures-util/0.3.30 \
    crate://crates.io/futures/0.3.30 \
    crate://crates.io/glob/0.3.1 \
    crate://crates.io/gloo-timers/0.2.6 \
    crate://crates.io/hashbrown/0.14.3 \
    crate://crates.io/hermit-abi/0.3.9 \
    crate://crates.io/home/0.5.9 \
    crate://crates.io/indexmap/2.2.6 \
    crate://crates.io/instant/0.1.12 \
    crate://crates.io/io-lifetimes/1.0.11 \
    crate://crates.io/itertools/0.12.1 \
    crate://crates.io/js-sys/0.3.69 \
    crate://crates.io/kv-log-macro/1.0.7 \
    crate://crates.io/lazy_static/1.4.0 \
    crate://crates.io/lazycell/1.3.0 \
    crate://crates.io/libc/0.2.153 \
    crate://crates.io/libcamera-sys/0.2.3 \
    crate://crates.io/libcamera/0.2.3 \
    crate://crates.io/libloading/0.8.3 \
    crate://crates.io/linux-raw-sys/0.3.8 \
    crate://crates.io/linux-raw-sys/0.4.13 \
    crate://crates.io/log/0.4.21 \
    crate://crates.io/memchr/2.7.1 \
    crate://crates.io/minimal-lexical/0.2.1 \
    crate://crates.io/nix/0.26.4 \
    crate://crates.io/nom/7.1.3 \
    crate://crates.io/num_enum/0.6.1 \
    crate://crates.io/num_enum_derive/0.6.1 \
    crate://crates.io/once_cell/1.19.0 \
    crate://crates.io/parking/2.2.0 \
    crate://crates.io/peeking_take_while/0.1.2 \
    crate://crates.io/pin-project-lite/0.2.13 \
    crate://crates.io/pin-utils/0.1.0 \
    crate://crates.io/piper/0.2.1 \
    crate://crates.io/pkg-config/0.3.30 \
    crate://crates.io/polling/2.8.0 \
    crate://crates.io/polling/3.5.0 \
    crate://crates.io/prettyplease/0.2.16 \
    crate://crates.io/proc-macro-crate/1.3.1 \
    crate://crates.io/proc-macro2/1.0.78 \
    crate://crates.io/quote/1.0.35 \
    crate://crates.io/regex-automata/0.4.6 \
    crate://crates.io/regex-syntax/0.8.2 \
    crate://crates.io/regex/1.10.3 \
    crate://crates.io/rustc-hash/1.1.0 \
    crate://crates.io/rustix/0.37.27 \
    crate://crates.io/rustix/0.38.31 \
    crate://crates.io/shlex/1.3.0 \
    crate://crates.io/signal-hook-registry/1.4.1 \
    crate://crates.io/slab/0.4.9 \
    crate://crates.io/smallvec/1.13.2 \
    crate://crates.io/socket2/0.4.10 \
    crate://crates.io/syn/1.0.109 \
    crate://crates.io/syn/2.0.52 \
    crate://crates.io/thiserror-impl/1.0.58 \
    crate://crates.io/thiserror/1.0.58 \
    crate://crates.io/toml_datetime/0.6.5 \
    crate://crates.io/toml_edit/0.19.15 \
    crate://crates.io/tracing-core/0.1.32 \
    crate://crates.io/tracing/0.1.40 \
    crate://crates.io/unicode-ident/1.0.12 \
    crate://crates.io/value-bag/1.7.0 \
    crate://crates.io/waker-fn/1.1.1 \
    crate://crates.io/wasm-bindgen-backend/0.2.92 \
    crate://crates.io/wasm-bindgen-futures/0.4.42 \
    crate://crates.io/wasm-bindgen-macro-support/0.2.92 \
    crate://crates.io/wasm-bindgen-macro/0.2.92 \
    crate://crates.io/wasm-bindgen-shared/0.2.92 \
    crate://crates.io/wasm-bindgen/0.2.92 \
    crate://crates.io/web-sys/0.3.69 \
    crate://crates.io/which/4.4.2 \
    crate://crates.io/winapi-i686-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi-x86_64-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi/0.3.9 \
    crate://crates.io/windows-sys/0.48.0 \
    crate://crates.io/windows-sys/0.52.0 \
    crate://crates.io/windows-targets/0.48.5 \
    crate://crates.io/windows-targets/0.52.4 \
    crate://crates.io/windows_aarch64_gnullvm/0.48.5 \
    crate://crates.io/windows_aarch64_gnullvm/0.52.4 \
    crate://crates.io/windows_aarch64_msvc/0.48.5 \
    crate://crates.io/windows_aarch64_msvc/0.52.4 \
    crate://crates.io/windows_i686_gnu/0.48.5 \
    crate://crates.io/windows_i686_gnu/0.52.4 \
    crate://crates.io/windows_i686_msvc/0.48.5 \
    crate://crates.io/windows_i686_msvc/0.52.4 \
    crate://crates.io/windows_x86_64_gnu/0.48.5 \
    crate://crates.io/windows_x86_64_gnu/0.52.4 \
    crate://crates.io/windows_x86_64_gnullvm/0.48.5 \
    crate://crates.io/windows_x86_64_gnullvm/0.52.4 \
    crate://crates.io/windows_x86_64_msvc/0.48.5 \
    crate://crates.io/windows_x86_64_msvc/0.52.4 \
    crate://crates.io/winnow/0.5.40 \
    "

S = "${WORKDIR}"
B = "${WORKDIR}/build"

do_install:append() {
    install -d ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/uvc-rtsp-streamer ${D}${bindir}
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/capture ${D}${bindir}/uvc-rtsp-capture
    install -m 0744 ${B}/target/${CARGO_TARGET_SUBDIR}/rtsp-loopback ${D}${bindir}/uvc-rtsp-loopback
}

