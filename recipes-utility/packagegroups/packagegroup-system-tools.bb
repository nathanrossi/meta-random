SUMMARY = "System tools for debugging, disks, network, etc."

inherit packagegroup

RDEPENDS:${PN} = " \
		iperf3 \
		iperf2 \
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

