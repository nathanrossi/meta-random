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
		linux-firmware-raspbian-bcm43430 \
		linux-firmware-raspbian-bcm43455 \
		"

KERNEL_MODULES = " \
		kernel-module-brcmfmac \
		kernel-module-snd-bcm2835 \
		kernel-module-vc4 \
		kernel-module-sdhci-iproc \
		kernel-module-uio-pdrv-genirq \
		kernel-module-smsc75xx \
		kernel-module-smsc95xx \
		kernel-module-lan78xx \
		"

RDEPENDS_${PN} = " \
		packagegroup-base-ext2 \
		packagegroup-base-vfat \
		packagegroup-base-usbhost \
		packagegroup-base-wifi \
		\
		packagegroup-system-tools \
		networkd-config \
		\
		${FIRMWARE_DEFAULT} \
		"

RRECOMMENDS_${PN} = " \
		${KERNEL_MODULES} \
		"

