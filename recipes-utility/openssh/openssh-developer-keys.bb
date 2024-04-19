DESCRIPTION = "Include known authorized_keys on the target for developer use"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COREBASE}/meta/COPYING.MIT;md5=3da9cfbcb788c80a0384361b4de20420"

do_configure[noexec] = "1"
do_compile[noexec] = "1"

do_install() {
    # install ssh public auth keys
    install -d -m0700 ${D}/home/root/.ssh
    cat ~/.ssh/authorized_keys | grep "local-hopping-key" > ${WORKDIR}/authorized_keys
    install -m0600 ${WORKDIR}/authorized_keys ${D}/home/root/.ssh/authorized_keys
}

FILES:${PN} += "/home/root/.ssh"
