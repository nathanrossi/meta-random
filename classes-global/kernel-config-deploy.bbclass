# Automatically deploy the kernel .config if the recipe inherits kernel
python () {
    if bb.data.inherits_class("deploy", d) and bb.data.inherits_class("kernel", d):
        bb.build.addtask("do_deploy_kernel_config", "do_build", "do_deploy", d)
        d.appendVar("SSTATETASKS", " do_deploy_kernel_config ")
}

DEPLOYCONFIGDIR = "${WORKDIR}/deploy-config-${PN}"
do_deploy_kernel_config[sstate-inputdirs] = "${DEPLOYCONFIGDIR}"
do_deploy_kernel_config[sstate-outputdirs] = "${DEPLOY_DIR_IMAGE}"
do_deploy_kernel_config[dirs] = "${B}"
do_deploy_kernel_config[cleandirs] = "${DEPLOYCONFIGDIR}"
do_deploy_kernel_config[stamp-extra-info] = "${MACHINE_ARCH}"
do_deploy_kernel_config() {
    # populate configuration of kernel into deploy directory
    install -m0644 ${B}/.config ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_NAME}.txt
    ln -sf kernel-config-${KERNEL_ARTIFACT_NAME}.txt ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_LINK_NAME}.txt
}
