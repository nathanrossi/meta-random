
UBOOT_CFG_REPLACE ?= ""

UBOOT_CFG_SERVERIP ?= "10.0.0.1"
UBOOT_CFG_IPADDR ?= "10.0.0.2"

UBOOT_ENV_SET[ethaddr] ?= "00:0a:35:00:01:22"
UBOOT_ENV_SET[machine] ?= "${MACHINE}"
UBOOT_ENV_SET[kernel] ?= "${KERNEL_IMAGETYPE}"
UBOOT_ENV_SET[fdt_high] ?= "0x20000000"
UBOOT_ENV_SET[initrd_high] ?= "0x20000000"

UBOOT_ENV_SET[kerneladdr] ?= "${UBOOT_LOAD_KERNEL_ADDR}"
UBOOT_ENV_SET[ramdiskaddr] ?= "${UBOOT_LOAD_RAMDISK_ADDR}"
UBOOT_ENV_SET[dtbaddr] ?= "${UBOOT_LOAD_DTB_ADDR}"

UBOOT_ENV_SET[boot] ?= " \
		tftpboot \${kerneladdr} \${kernel} && \
		tftpboot \${ramdiskaddr} core-image-minimal-\${machine}.cpio.gz.u-boot && \
		tftpboot \${dtbaddr} \${machine} && \
		bootm \${kerneladdr} \${ramdiskaddr} \${dtbaddr} \
		"

# zynq
UBOOT_CFG_PATH_zynq = "${S}/include/configs/zynq-common.h"
UBOOT_LOAD_KERNEL_ADDR_zynq ?= "0x2080000"
UBOOT_LOAD_RAMDISK_ADDR_zynq ?= "0x4000000"
UBOOT_LOAD_DTB_ADDR_zynq ?= "0x2000000"

# microblaze
UBOOT_CFG_PATH_microblaze = "${S}/include/configs/microblaze-generic.h"
#UBOOT_LOAD_KERNEL_ADDR_kc705-trd-microblazeel = "0x2080000"
#UBOOT_LOAD_RAMDISK_ADDR_kc705-trd-microblazeel = "0x4000000"
#UBOOT_LOAD_DTB_ADDR_kc705-trd-microblazeel = "0x2000000"

python do_magic_env_replace() {
    import re
    environment = {}
    config_h_path = d.getVar('UBOOT_CFG_PATH', True)
    config_h_path = config_h_path if config_h_path and os.path.exists(config_h_path) else None
    if d.getVar("UBOOT_CFG_REPLACE", True) == "1" and config_h_path:
        bb.warn("u-boot environment has overridden/injected content")

        for i in d.getVarFlags("UBOOT_ENV_SET"):
            formatted = d.getVarFlag("UBOOT_ENV_SET", i, True)
            formatted = re.sub("\\\\\$", "$", formatted)
            formatted = re.sub("\\t", "", formatted)
            formatted = formatted.strip(" ")
            environment[i] = formatted

        defenv = ""
        for i in environment.items():
            defenv += "\"%s=%s\\\\0\" \\\n" % (i[0], i[1])

        with open(config_h_path, 'r') as f:
            config_h = f.read()

        if re.search("#define\s*?CONFIG_EXTRA_ENV_SETTINGS", config_h, re.DOTALL | re.MULTILINE | re.IGNORECASE):
            config_h = re.sub("#define\s*?CONFIG_EXTRA_ENV_SETTINGS.*?^$",
                    "#define CONFIG_EXTRA_ENV_SETTINGS \\\n" + defenv, config_h,
                    flags = re.DOTALL | re.MULTILINE | re.IGNORECASE)
        else:
            # CONFIG_EXTRA_ENV_SETTINGS not already defined, add after PREBOOT
            config_h = re.sub("(#define\s*?CONFIG_PREBOOT.*?)^$",
                    "\g<1>\n#define CONFIG_EXTRA_ENV_SETTINGS \\\n" + defenv, config_h,
                    flags = re.DOTALL | re.MULTILINE | re.IGNORECASE)

        config_h = re.sub("(#define\s*?CONFIG_SYS_MAXARGS).*", "\g<1> 32", config_h)
        config_h = re.sub("(#define\s*?CONFIG_BOOTCOMMAND).*", "\g<1> \"run boot\"", config_h)
        config_h = re.sub("(#define\s*?CONFIG_BOOTDELAY).*", "\g<1> 0", config_h)
        config_h = re.sub("(#define\s*?CONFIG_IPADDR).*", "\g<1> %s" % d.getVar("UBOOT_CFG_IPADDR", True), config_h)
        config_h = re.sub("(#define\s*?CONFIG_SERVERIP).*", "\g<1> %s" % d.getVar("UBOOT_CFG_SERVERIP", True), config_h)

        with open(config_h_path, 'w') as f:
            f.write(config_h)
}
addtask do_magic_env_replace after do_unpack before do_configure

do_deploy_append () {
	if [ -f ${S}/spl/u-boot-spl.bin ]; then
		install ${S}/spl/u-boot-spl.bin ${DEPLOYDIR}/u-boot-spl.bin
	fi
}

