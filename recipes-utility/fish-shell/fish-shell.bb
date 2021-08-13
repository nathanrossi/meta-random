SUMMARY = "The user-friendly command line shell."
HOMEPAGE = "https://fishshell.com"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=937511e42dab6bf5fc0786f06fd377a8"

SRC_URI = "git://github.com/fish-shell/fish-shell.git;protocol=https"
SRCREV = "0314b0f1d94299903dc193f2f57529b56a42a96e"
PV = "3.1.2+git${SRCPV}"

S = "${WORKDIR}/git"

DEPENDS += "ncurses"

inherit cmake

do_install:append:class-nativesdk() {
    # fish's relocatable detection assumes the path relative to the binary
    # "../etc" is the sysconfdir. symlink so that it works
    ln -s ../etc ${D}${prefix}/etc
}

BBCLASSEXTEND = "native nativesdk"

FILES:${PN} += "${datadir}/fish"
FILES:${PN} += "${prefix}/etc"

