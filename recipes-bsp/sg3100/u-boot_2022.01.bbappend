FILESEXTRAPATHS:prepend := "${THISDIR}/u-boot:"

SRC_URI:append:sg3100 = " \
    file://0001-ddr-marvell-a38x-Import-master-DDR3-4-training-code.patch \
    file://0002-Add-support-for-Netgate-SG3100.patch \
    "

