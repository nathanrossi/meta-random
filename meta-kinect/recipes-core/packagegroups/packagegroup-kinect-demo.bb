SUMMARY = "Kinect Demo"
LICENSE = "MIT"
PR = "r1"

inherit packagegroup

RDEPENDS_${PN} = " \
		libfreenect \
		openssh \
		"