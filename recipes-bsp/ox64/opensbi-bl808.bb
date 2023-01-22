require recipes-bsp/opensbi/opensbi_1.1.bb

SRC_URI = "git://github.com/smaeul/opensbi;protocol=https;branch=bl808"
SRCREV = "1767f7f5473b608cf8cfc66c5799a8f61c72c16d"

PV = "1.1+bl808+git${SRCPV}"

# depend on the u-boot binary
do_compile[depends] += "u-boot-bl808:do_deploy"

EXTRA_OEMAKE += "FW_FDT_PATH=${DEPLOY_DIR_IMAGE}/u-boot-bl808-d0-ox64.dtb"
EXTRA_OEMAKE += "FW_PAYLOAD_PATH=${DEPLOY_DIR_IMAGE}/u-boot.bin-d0"
EXTRA_OEMAKE += "FW_PAYLOAD_OFFSET=0x100000"
EXTRA_OEMAKE += "FW_TEXT_START=0x50000000"

PACKAGE_ARCH = "${MACHINE_ARCH}"

do_compile:append() {
    # strip fw_payload.elf
    ${STRIP} ${B}/build/platform/generic/firmware/fw_payload.elf
}

INSANE_SKIP:${PN} = "already-stripped"
