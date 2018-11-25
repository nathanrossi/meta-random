SUMMARY = "Configuration for systemd-networkd"
DESCRIPTION = "Provide systemd-networkd configuration files"
SECTION = "bsp"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

PACKAGE_ARCH = "${MACHINE_ARCH}"
S = "${WORKDIR}"

FILESEXTRAPATHS_append := "${THISDIR}/files:"
SRC_URI = "file://dhcp.network"

FILES_${PN} += "${systemd_unitdir}"

do_install () {
    for i in $(find ${S} -name "*.network" -or -name "*.netdev"); do
        install -Dm 0644 $i ${D}${sysconfdir}/systemd/network/$(basename $i)
    done
}

