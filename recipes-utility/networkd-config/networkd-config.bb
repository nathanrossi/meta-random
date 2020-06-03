SUMMARY = "Configuration for systemd-networkd"
DESCRIPTION = "Provide systemd-networkd configuration files"
SECTION = "bsp"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

PACKAGE_ARCH = "${MACHINE_ARCH}"
S = "${WORKDIR}"

WIRELESS_NETWORK ??= ""
WIRELESS ??= ""

RDEPENDS_${PN} += "wpa-supplicant"

SRC_URI += "file://wpa_supplicant-interface@.service"

FILES_${PN} += "${systemd_unitdir}"
RDEPENDS_${PN} += "glib-2.0-utils"

do_configure[noexec] = "1"
do_compile[noexec] = "1"

python () {
    if d.getVar("WIRELESS_NETWORK"):
        bb.build.addtask("do_generate_wpa_supplicant", "do_install", "do_compile", d)
        d.appendVar("CONFFILES_${PN}", " ${sysconfdir}/wpa_supplicant/wpa_supplicant-common.conf")
}

python do_generate_wpa_supplicant () {
    networks = []
    for i in (d.getVar("WIRELESS_NETWORK") or "").split():
        ssid, psk = (i, None) if ";" not in i else i.split(";")
        networks.append((ssid, psk))

    with open(d.expand("${WORKDIR}/wpa_supplicant-common.conf"), "w") as f:
        for ssid, psk in networks:
            f.write("network={\n")
            f.write("  ssid=\"{0}\"\n".format(ssid))
            f.write("  psk=\"{0}\"\n".format(psk))
            f.write("}\n")
}

do_install () {
    cat << EOF > ${WORKDIR}/default-dhcp.network
[Match]
Name=eth*
Name=eno*
Name=enp*
Name=enx*
Name=wlan*
Name=wlp*

[Network]
DHCP=ipv4
LLMNR=false
[DHCPv4]
ClientIdentifier=mac
EOF
    install -D ${WORKDIR}/default-dhcp.network ${D}${sysconfdir}/systemd/network/default-dhcp.network

    if [ -e ${WORKDIR}/wpa_supplicant-common.conf ]; then
        install -Dm 0644 ${WORKDIR}/wpa_supplicant-common.conf ${D}${sysconfdir}/wpa_supplicant/wpa_supplicant-common.conf
    fi
    install -Dm 644 ${WORKDIR}/wpa_supplicant-interface@.service ${D}${systemd_unitdir}/system/wpa_supplicant-interface@.service
    for i in ${WIRELESS}; do
        # enable service
        install -d ${D}${sysconfdir}/systemd/system/multi-user.target.wants
        ln -s ${systemd_unitdir}/system/wpa_supplicant-interface@.service \
            ${D}${sysconfdir}/systemd/system/multi-user.target.wants/wpa_supplicant-interface@$i.service
    done
}

