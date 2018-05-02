SUMMARY = "System tools for debugging, disks, network, etc."

inherit packagegroup

RDEPENDS_${PN} = " \
		iperf3 \
		iputils \
		iproute2 \
		tcpdump \
		ethtool \
		\
		usbutils \
		pciutils \
		i2c-tools \
		minicom \
		\
		strace \
		file \
		util-linux e2fsprogs \
		"

