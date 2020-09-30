DESCRIPTION = "Disk image for core-image-minimal (used to access deployed core-image-minimal)"

PACKAGE_INSTALL = ""
IMAGE_INSTALL = ""
IMAGE_LINGUAS = ""

LICENSE = "MIT"

inherit core-image

IMAGE_FSTYPES = "wic"
WKS_FILES = "vfat-bootonly.wks"

do_image_wic[depends] += "core-image-minimal:do_image_complete"

GENERATED_DEPLOY_DIR = "${WORKDIR}/image-bootfiles"
DEPLOY_DIR_IMAGE_RELATIVE = "${@os.path.relpath(d.getVar("GENERATED_DEPLOY_DIR"), d.getVar("DEPLOY_DIR_IMAGE"))}"

python do_generate_deployables() {
    os.makedirs(d.getVar("GENERATED_DEPLOY_DIR"))
}
addtask generate_deployables before do_image_wic after do_image

# include the initramfs cpio and generate rpi cmdline.txt and update config.txt
# to include initramfs
IMAGE_BOOT_FILES_append_rpi = " \
        core-image-minimal-${MACHINE}.cpio.gz;initramfs.gz \
        ${DEPLOY_DIR_IMAGE_RELATIVE}/cmdline.txt;cmdline.txt \
        ${DEPLOY_DIR_IMAGE_RELATIVE}/config.txt;config.txt \
        "

python do_generate_deployables_append_rpi() {
    cmdline = d.expand("${GENERATED_DEPLOY_DIR}/cmdline.txt")
    with open(cmdline, "w") as f:
        f.write("dwc_otg.lpm_enable=0 console=tty0")

    configtxt = d.expand("${GENERATED_DEPLOY_DIR}/config.txt")
    with open(configtxt, "w") as f:
        with open(d.expand("${DEPLOY_DIR_IMAGE}/bootfiles/config.txt"), "r") as s:
            for i in s:
                f.write(i)
        # something broke followkernel, so point it at 0x08000000 aka @128M
        #RPI_EXTRA_CONFIG_append_rpi = "initramfs initramfs.gz 0x08000000\n"
        f.write("initramfs initramfs.gz followkernel\n")
}

