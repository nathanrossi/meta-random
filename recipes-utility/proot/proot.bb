SUMMARY = "user-space chroot, mount --bind and binfmt_misc"
HOMEPAGE = "https://proot-me.github.io/"
SECTION = "tools"

LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://COPYING;md5=b234ee4d69f5fce4486a80fdaf4a4263"

PV = "5.1.0+git${SRCPV}"

SRC_URI = "git://github.com/proot-me/proot;protocol=https"
SRCREV = "803e54d8a1b3d513108d3fc413ba6f7c80220b74"

DEPENDS += "libtalloc"

S = "${WORKDIR}/git"

PARALLEL_MAKE = ""
EXTRA_OEMAKE += "'PREFIX=${exec_prefix}'"

do_compile() {
    oe_runmake -C ${S}/src
}

do_install() {
    oe_runmake -C ${S}/src 'DESTDIR=${D}' install
}

BBCLASSEXTEND = "native nativesdk"
