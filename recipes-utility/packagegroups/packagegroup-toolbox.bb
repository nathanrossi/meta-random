SUMMARY = "Toolbox base system packagegroup"

PACKAGE_ARCH = "${MACHINE_ARCH}"

inherit packagegroup

# firmware
FIRMWARE_DEFAULT = " \
		linux-firmware-rtl8188 \
		linux-firmware-rtl8192cu \
		linux-firmware-rtl8192ce \
		linux-firmware-rtl8192su \
		linux-firmware-rtl8723 \
		linux-firmware-rtl8821 \
		"

RDEPENDS_${PN} = " \
		coreutils findutils \
		tar xz unzip \
		${@bb.utils.contains('DISTRO_FEATURES', 'systemd', 'systemd-analyze', '', d)} \
		\
		packagegroup-base \
		\
		packagegroup-system-tools \
		networkd-config \
		\
		python3-core \
		python3-pip \
		python3-spidev \
		\
		flashrom \
		sigrok-cli \
		\
		${FIRMWARE_DEFAULT} \
		"

RRECOMMENDS_${PN} = " \
		${MACHINE_EXTRA_RRECOMMENDS} \
		"

