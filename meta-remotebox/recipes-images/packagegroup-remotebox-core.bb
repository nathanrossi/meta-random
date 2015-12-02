DESCRIPTION = "Core packages for a remotebox"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COREBASE}/LICENSE;md5=4d92cd373abda3937c2bc47fbc49d690"

PACKAGE_ARCH = "${MACHINE_ARCH}"
inherit packagegroup

RDEPENDS_${PN} = "\
		coreutils \
		bash \
		sudo \
		unzip \
		tar \
		xz \
		cpio \
		gzip \
		sed \
		grep \
		rsync \
		\
		python \
		python-modules \
		\
		git \
		\
		tmux \
		vim \
		\
		usbutils \
		pciutils \
		util-linux \
		dosfstools \
		e2fsprogs \
		\
		openssh-sshd \
		openssh \
		iproute2 \
		iptables \
		\
		iw \
		wpa-supplicant \
		crda \
		\
		iperf3 \
		minicom \
		\
		kernel-modules \
		linux-firmware-iwlwifi \
		linux-firmware-rtl \
		"

