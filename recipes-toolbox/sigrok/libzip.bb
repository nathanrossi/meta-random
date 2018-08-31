SUMMARY = "A C library for reading, creating, and modifying zip archives."
HOMEPAGE = "https://libzip.org"
SECTION = "libs"

LICENSE = "BSD-3-Clause"
LIC_FILES_CHKSUM = "file://LICENSE;md5=01f8b1b8da6403739094396e15b1e722"

DEPENDS += "zlib bzip2 openssl"

SRC_URI = "git://github.com/nih-at/libzip;protocol=https"
SRCREV = "25e23dd9ed04689f0fcb7c4c75ecc35932226c38"

S = "${WORKDIR}/git"

inherit cmake

BBCLASSEXTEND = "native nativesdk"
