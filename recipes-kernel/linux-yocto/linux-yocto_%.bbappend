FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append:testbox = " file://testbox-config.cfg "

do_deploy:append() {
    # populate configuration of kernel into deploy directory
    install -m0644 .config ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_NAME}.txt
    ln -sf kernel-config-${KERNEL_ARTIFACT_NAME}.txt ${DEPLOYDIR}/kernel-config-${KERNEL_ARTIFACT_LINK_NAME}.txt
}
