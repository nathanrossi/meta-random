SUMMARY = "Device Tree Blob Overlay Configuration File System"
LICENSE = "BSD-2-Clause"
LIC_FILES_CHKSUM = "file://LICENSE;md5=6e83d63de93384e6cce0fd3632041d91"

SRC_URI = "git://github.com/ikwzm/dtbocfg;protocol=https;branch=master"
SRCREV = "2d5593ba4fea35b75fce716a5d9538f984493cbd"

S = "${WORKDIR}/git"

inherit module
