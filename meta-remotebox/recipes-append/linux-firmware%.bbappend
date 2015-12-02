
PACKAGES =+ "${PN}-iwlwifi"
ALLOW_EMPTY_${PN}-iwlwifi = "1"
RDEPENDS_${PN}-iwlwifi += " \
		${PN}-iwlwifi-license ${PN}-iwlwifi-135-6 \
		${PN}-iwlwifi-3160-7 ${PN}-iwlwifi-3160-8 ${PN}-iwlwifi-3160-9 \
		${PN}-iwlwifi-6000-4 ${PN}-iwlwifi-6000g2a-5 ${PN}-iwlwifi-6000g2a-6 \
		${PN}-iwlwifi-6000g2b-5 ${PN}-iwlwifi-6000g2b-6 \
		${PN}-iwlwifi-6050-4 ${PN}-iwlwifi-6050-5 \
		${PN}-iwlwifi-7260-7 ${PN}-iwlwifi-7260-8 ${PN}-iwlwifi-7260-9 \
		${PN}-iwlwifi-7260-10 ${PN}-iwlwifi-7260-12 ${PN}-iwlwifi-7260-13 \
		${PN}-iwlwifi-7265-8 ${PN}-iwlwifi-7265-9 \
		"

PACKAGES =+ "${PN}-rtl"
ALLOW_EMPTY_${PN}-rtl = "1"
RDEPENDS_${PN}-rtl += " \
		${PN}-rtl-license ${PN}-rtl8192cu ${PN}-rtl8192ce ${PN}-rtl8192su \
		"

PACKAGES =+ " \
		${PN}-iwlwifi-7260-10 \
		${PN}-iwlwifi-7260-12 \
		${PN}-iwlwifi-7260-13 \
		"

LICENSE_${PN}-iwlwifi-7260-10   = "Firmware-iwlwifi_firmware"
LICENSE_${PN}-iwlwifi-7260-12   = "Firmware-iwlwifi_firmware"
LICENSE_${PN}-iwlwifi-7260-13   = "Firmware-iwlwifi_firmware"

FILES_${PN}-iwlwifi-7260-10 = "/lib/firmware/iwlwifi-7260-10.ucode"
FILES_${PN}-iwlwifi-7260-12 = "/lib/firmware/iwlwifi-7260-12.ucode"
FILES_${PN}-iwlwifi-7260-13 = "/lib/firmware/iwlwifi-7260-13.ucode"

RDEPENDS_${PN}-iwlwifi-7260-10 = "${PN}-iwlwifi-license"
RDEPENDS_${PN}-iwlwifi-7260-12 = "${PN}-iwlwifi-license"
RDEPENDS_${PN}-iwlwifi-7260-13 = "${PN}-iwlwifi-license"

