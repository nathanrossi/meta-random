SUMMARY = "libfreenect for USB Kinect control and access"
DESCRIPTION = "libfreenect Kinect library"
HOMEPAGE = "https://github.com/OpenKinect/libfreenect"
SECTION = "libs"

LICENSE = "Apache-2.0 | GPLv2"

LIC_FILES_CHKSUM = " \
		file://APACHE20;md5=3b83ef96387f14655fc854ddc3c6bd57 \
		file://GPL2;md5=eb723b61539feef013de476e68b5c50a \
		"

DEPENDS = "libusb"
PR = "r0"

inherit cmake pkgconfig

SRC_URI = "git://github.com/OpenKinect/libfreenect;protocol=git"
SRCREV = "5eed1f061da229aab600471a4a85e83844151af9"
S = "${WORKDIR}/git"

EXTRA_OECMAKE = "-DBUILD_EXAMPLES=no"

PACKAGES += "libfakenect libfakenect-dbg"
INSANE_SKIP_libfakenect = "dev-so"

FILES_libfakenect += "/usr/lib/fakenect/libfreenect.so*"
FILES_libfakenect-dbg += "/usr/lib/fakenect/.debug/"