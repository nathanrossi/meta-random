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

SRC_URI[autocfg-1.1.0.sha256sum] = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"
SRC_URI[bitflags-1.3.2.sha256sum] = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"
SRC_URI[cfg-if-0.1.10.sha256sum] = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822"
SRC_URI[cfg-if-1.0.0.sha256sum] = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"
SRC_URI[fuchsia-zircon-sys-0.3.3.sha256sum] = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7"
SRC_URI[fuchsia-zircon-0.3.3.sha256sum] = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82"
SRC_URI[futures-channel-0.3.21.sha256sum] = "c3083ce4b914124575708913bca19bfe887522d6e2e6d0952943f5eac4a74010"
SRC_URI[futures-core-0.3.21.sha256sum] = "0c09fd04b7e4073ac7156a9539b57a484a8ea920f79c7c675d05d289ab6110d3"
SRC_URI[futures-executor-0.3.21.sha256sum] = "9420b90cfa29e327d0429f19be13e7ddb68fa1cccb09d65e5706b8c7a749b8a6"
SRC_URI[futures-io-0.3.21.sha256sum] = "fc4045962a5a5e935ee2fdedaa4e08284547402885ab326734432bed5d12966b"
SRC_URI[futures-macro-0.3.21.sha256sum] = "33c1e13800337f4d4d7a316bf45a567dbcb6ffe087f16424852d97e97a91f512"
SRC_URI[futures-sink-0.3.21.sha256sum] = "21163e139fa306126e6eedaf49ecdb4588f939600f0b1e770f4205ee4b7fa868"
SRC_URI[futures-task-0.3.21.sha256sum] = "57c66a976bf5909d801bbef33416c41372779507e7a6b3a5e25e4749c58f776a"
SRC_URI[futures-util-0.3.21.sha256sum] = "d8b7abd5d659d9b90c8cba917f6ec750a74e2dc23902ef9cd4cc8c8b22e6036a"
SRC_URI[futures-0.3.21.sha256sum] = "f73fe65f54d1e12b726f517d3e2135ca3125a437b6d998caf1962961f7172d9e"
SRC_URI[iovec-0.1.4.sha256sum] = "b2b3ea6ff95e175473f8ffe6a7eb7c00d054240321b84c57051175fe3c1e075e"
SRC_URI[kernel32-sys-0.2.2.sha256sum] = "7507624b29483431c0ba2d82aece8ca6cdba9382bff4ddd0f7490560c056098d"
SRC_URI[libc-0.2.126.sha256sum] = "349d5a591cd28b49e1d1037471617a32ddcda5731b99419008085f72d5a53836"
SRC_URI[log-0.4.17.sha256sum] = "abb12e687cfb44aa40f41fc3978ef76448f9b6038cad6aef4259d3c095a2382e"
SRC_URI[memchr-2.5.0.sha256sum] = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d"
SRC_URI[memoffset-0.6.5.sha256sum] = "5aa361d4faea93603064a027415f07bd8e1d5c88c9fbf68bf56a285428fd79ce"
SRC_URI[mio-0.6.23.sha256sum] = "4afd66f5b91bf2a3bc13fad0e21caedac168ca4c707504e75585648ae80e4cc4"
SRC_URI[miow-0.2.2.sha256sum] = "ebd808424166322d4a38da87083bfddd3ac4c131334ed55856112eb06d46944d"
SRC_URI[net2-0.2.37.sha256sum] = "391630d12b68002ae1e25e8f974306474966550ad82dac6886fb8910c19568ae"
SRC_URI[nix-0.24.1.sha256sum] = "8f17df307904acd05aa8e32e97bb20f2a0df1728bbc2d771ae8f9a90463441e9"
SRC_URI[pin-project-lite-0.2.9.sha256sum] = "e0a7ae3ac2f1173085d398531c705756c94a4c56843785df85a60c1a0afac116"
SRC_URI[pin-utils-0.1.0.sha256sum] = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"
SRC_URI[proc-macro2-1.0.40.sha256sum] = "dd96a1e8ed2596c337f8eae5f24924ec83f5ad5ab21ea8e455d3566c69fbcaf7"
SRC_URI[quote-1.0.20.sha256sum] = "3bcdf212e9776fbcb2d23ab029360416bb1706b1aea2d1a5ba002727cbcab804"
SRC_URI[slab-0.4.6.sha256sum] = "eb703cfe953bccee95685111adeedb76fabe4e97549a58d16f03ea7b9367bb32"
SRC_URI[syn-1.0.98.sha256sum] = "c50aef8a904de4c23c788f104b7dddc7d6f79c647c7c8ce4cc8f73eb0ca773dd"
SRC_URI[unicode-ident-1.0.1.sha256sum] = "5bd2fe26506023ed7b5e1e315add59d6f584c621d037f9368fea9cfb988f368c"
SRC_URI[winapi-build-0.1.1.sha256sum] = "2d315eee3b34aca4797b2da6b13ed88266e6d612562a0c46390af8299fc699bc"
SRC_URI[winapi-i686-pc-windows-gnu-0.4.0.sha256sum] = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"
SRC_URI[winapi-x86_64-pc-windows-gnu-0.4.0.sha256sum] = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"
SRC_URI[winapi-0.2.8.sha256sum] = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a"
SRC_URI[winapi-0.3.9.sha256sum] = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"
SRC_URI[ws2_32-sys-0.2.1.sha256sum] = "d59cefebd0c892fa2dd6de581e937301d8552cb44489cdff035c6187cb63fa5e"

S = "${WORKDIR}/sources"
B = "${WORKDIR}/build"
UNPACKDIR = "${S}"

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
