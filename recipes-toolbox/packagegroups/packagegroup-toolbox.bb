SUMMARY = "Toolbox base system packagegroup"

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
		systemd-analyze \
		\
		packagegroup-base-ext2 \
		packagegroup-base-vfat \
		packagegroup-base-usbhost \
		packagegroup-base-wifi \
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

