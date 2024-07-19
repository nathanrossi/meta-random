FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append:testbox = " file://testbox-config.cfg "
KERNEL_FEATURES:append:genericx86-64:testbox = " bsp/amd-x86/amd-x86-64.scc "
KERNEL_FEATURES:append:genericx86-64:testbox = " bsp/intel-x86/intel-x86.scc "

SRC_URI:append:tiny = " file://testbox-config.cfg "
# KERNEL_FEATURES:append:genericx86-64:tiny = " bsp/amd-x86/amd-x86-64.scc "
# KERNEL_FEATURES:append:genericx86-64:tiny = " bsp/intel-x86/intel-x86.scc "
