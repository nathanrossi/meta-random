SUMMARY = "RISC-V Open Source Supervisor Binary Interface (OpenSBI)"
DESCRIPTION = "OpenSBI aims to provide an open-source and extensible implementation of the RISC-V SBI specification for a platform specific firmware (M-mode) and a general purpose OS, hypervisor or bootloader (S-mode or HS-mode). OpenSBI implementation can be easily extended by RISC-V platform or System-on-Chip vendors to fit a particular hadware configuration."
HOMEPAGE = "https://github.com/riscv/opensbi"
LICENSE = "BSD-2-Clause"
LIC_FILES_CHKSUM = "file://COPYING.BSD;md5=42dd9555eb177f35150cf9aa240b61e5"

require recipes-bsp/opensbi/opensbi-payloads.inc

inherit autotools-brokensep deploy

SRC_URI = "git://github.com/smaeul/opensbi;protocol=https;branch=bl808"
SRCREV = "1767f7f5473b608cf8cfc66c5799a8f61c72c16d"

PV = "1.1+bl808+git${SRCPV}"

S = "${WORKDIR}/git"

EXTRA_OEMAKE += "PLATFORM=${RISCV_SBI_PLAT} I=${D} FW_PIC=n CLANG_TARGET= "
# If RISCV_SBI_PAYLOAD is set then include it as a payload
EXTRA_OEMAKE:append = " ${@riscv_get_extra_oemake_image(d)}"
EXTRA_OEMAKE:append = " ${@riscv_get_extra_oemake_fdt(d)}"

# Required if specifying a custom payload
do_compile[depends] += "${@riscv_get_do_compile_depends(d)}"

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

do_install:append() {
    # In the future these might be required as a dependency for other packages.
    # At the moment just delete them to avoid warnings
    rm -r ${D}/include
    rm -r ${D}/lib*
    rm -r ${D}/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/payloads
}

do_deploy () {
    install -m 755 ${D}/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_payload.* ${DEPLOYDIR}/
    install -m 755 ${D}/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_jump.* ${DEPLOYDIR}/
    install -m 755 ${D}/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_dynamic.* ${DEPLOYDIR}/
}

addtask deploy before do_build after do_install

INSANE_SKIP:${PN} = "already-stripped"

FILES:${PN} += "/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_jump.*"
FILES:${PN} += "/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_payload.*"
FILES:${PN} += "/share/opensbi/*/${RISCV_SBI_PLAT}/firmware/fw_dynamic.*"

COMPATIBLE_HOST = "(riscv64|riscv32).*"
INHIBIT_PACKAGE_STRIP = "1"

SECURITY_CFLAGS = ""
