FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append:testbox = " file://testbox-config.cfg "
KERNEL_FEATURES:append:genericx86-64:testbox = " bsp/amd-x86/amd-x86-64.scc "
KERNEL_FEATURES:append:genericx86-64:testbox = " bsp/intel-x86/intel-x86.scc "

do_deploy:append() {
    # populate configuration of kernel into deploy directory
    install -m0644 .config ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_NAME}.txt
    ln -sf kernel-config-${KERNEL_ARTIFACT_NAME}.txt ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_LINK_NAME}.txt
}
