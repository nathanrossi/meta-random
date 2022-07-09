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

do_generate_deployables[dirs] += "${GENERATED_DEPLOY_DIR}"
python do_generate_deployables() {
}
addtask generate_deployables before do_image_wic after do_image

# include the initramfs cpio and generate rpi cmdline.txt and update config.txt
# to include initramfs
IMAGE_BOOT_FILES:append:rpi = " \
        core-image-minimal-${MACHINE}.cpio.gz;initramfs.gz \
        ${DEPLOY_DIR_IMAGE_RELATIVE}/cmdline.txt;cmdline.txt \
        ${DEPLOY_DIR_IMAGE_RELATIVE}/config.txt;config.txt \
        "

python do_generate_deployables:append:rpi() {
    cmdline = d.expand("${GENERATED_DEPLOY_DIR}/cmdline.txt")
    with open(cmdline, "w") as f:
        if d.getVar("ENABLE_UART") == "1":
            f.write("dwc_otg.lpm_enable=0 console=serial0,115200")
        else:
            f.write("dwc_otg.lpm_enable=0 console=tty0")

    configtxt = d.expand("${GENERATED_DEPLOY_DIR}/config.txt")
    with open(configtxt, "w") as f:
        with open(d.expand("${DEPLOY_DIR_IMAGE}/bootfiles/config.txt"), "r") as s:
            for i in s:
                f.write(i)
        # something broke followkernel, so point it at 0x08000000 aka @128M
        # RPI_EXTRA_CONFIG:append:rpi = "initramfs initramfs.gz 0x08000000\n"
        f.write("initramfs initramfs.gz followkernel\n")
}

do_generate_boot_tarball[depends] += "core-image-minimal:do_image_complete"
python do_generate_boot_tarball() {
    args = ["tar", "-czhf", d.expand("${IMGDEPLOYDIR}/${IMAGE_NAME}.boot.tar.gz")]
    import glob
    for i in d.getVar("IMAGE_BOOT_FILES").split():
        parts = i.split(";", 1)
        abspath = os.path.abspath(os.path.join(d.getVar("DEPLOY_DIR_IMAGE"), parts[0]))
        for g in glob.glob(abspath):
            src = os.path.relpath(g, d.getVar("TMPDIR"))
            dst = os.path.basename(g) if len(parts) == 1 else parts[1]
            args.append("--transform=s#{}#{}#".format(src, dst))
            args.append(src)

    import subprocess
    subprocess.run(args, check = True, cwd = d.getVar("TMPDIR"), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

    # symlink newest
    target = d.expand("${IMAGE_NAME}.boot.tar.gz")
    symlink = d.expand("${IMGDEPLOYDIR}/${IMAGE_LINK_NAME}.boot.tar.gz")
    if os.path.islink(symlink):
        os.remove(symlink)
    os.symlink(target, symlink)
}
addtask generate_boot_tarball before do_image_wic after do_generate_deployables
