SUMMARY = "System tools for debugging, disks, network, etc."

inherit packagegroup

RDEPENDS_${PN} = " \
		file \
		iperf3 \
		usbutils \
		i2c-tools \
		ethtool \
		pciutils \
		util-linux e2fsprogs \
		strace \
		minicom \
		"

