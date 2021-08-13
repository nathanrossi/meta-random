SUMMARY = "Input-Event device cloning utility. Use it together with ssh/netcat/... to share input devices among different machines."
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://LICENSE;md5=751419260aa954499f7abaabaa882bbe"

S = "${WORKDIR}/git"

SRC_URI = "git://github.com/Blub/netevent;procotol=https"
SRCREV = "ddd330f0dc956a95a111c58ad10546071058e4c1"

EXTRA_OEMAKE += "PREFIX=${prefix}"

do_configure:append() {
    for file in ${S}/src/reader.cpp ${S}/src/writer.cpp; do
        sed -i 's/ev\.time\.tv_sec/ev.input_event_sec/g' $file
        sed -i 's/ev\.time\.tv_usec/ev.input_event_usec/g' $file
    done
}

do_install() {
    oe_runmake install DESTDIR=${D}
}
