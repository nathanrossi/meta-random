SUMMARY = "Utility base system packagegroup"

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

RDEPENDS:${PN} = " \
		packagegroup-base \
		\
		python3-core \
		python3-pip \
		python3-spidev \
		\
		dtbocfg \
		\
		${FIRMWARE_DEFAULT} \
		"

RRECOMMENDS:${PN} = " \
		${MACHINE_EXTRA_RRECOMMENDS} \
		"

