SUMMARY = "Configuration for systemd-networkd"
DESCRIPTION = "Provide systemd-networkd configuration files"
SECTION = "bsp"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"

PACKAGE_ARCH = "${MACHINE_ARCH}"
S = "${WORKDIR}"

CONFIG_FILES ??= ""
WIRELESS ??= ""

SRC_URI += " \
        file://wpa_supplicant-interface@.service \
        "

FILES_${PN} += "${systemd_unitdir}"
RDEPENDS_${PN} += "glib-2.0-utils"

python () {
    import re
    for i in d.getVar("FILESPATH").split(":"):
        if not os.path.isdir(i):
            continue
        for f in os.listdir(i):
            name, ext = os.path.splitext(f)
            if ext in [".network", ".netdev"]:
                d.appendVar("SRC_URI", " file://{0}".format(f))
                d.appendVar("CONFFILES_${PN}", " ${sysconfdir}/systemd/network/" + f)
                d.appendVar("CONFIG_FILES", " " + f)

                # auto wifi config for .network files with matching "Name=wlp/wlan..."
                with open(os.path.join(i, f), "r") as f:
                    for n in re.finditer("^Name=(.*?)$", f.read(), re.MULTILINE):
                        name = n.group(1)
                        if "*" not in name and name.startswith("wlan") or name.startswith("wlp"):
                            d.appendVar("WIRELESS", " " + name)
                            d.appendVar("CONFFILES_${PN}", " ${sysconfdir}" + "/wpa_supplicant/wpa_supplicant-{0}.conf".format(f))

    if d.getVar("DEFAULT_WIFI") and d.getVar("WIRELESS"):
        bb.build.addtask("do_generate_wpa_supplicant", "do_install", "do_compile", d)
        d.appendVar("CONFFILES_${PN}", " ${sysconfdir}/wpa_supplicant/wpa_supplicant-common.conf")
}

python do_generate_wpa_supplicant () {
    ssid, psk, *_ = (d.getVar("DEFAULT_WIFI") + ";").split(";")
    with open(d.expand("${WORKDIR}/wpa_supplicant-common.conf"), "w") as f:
        f.write("network={\n")
        f.write("ssid=\"{0}\"\n".format(ssid))
        f.write("psk=\"{0}\"\n".format(psk))
        f.write("}\n")
}

do_install () {
    for i in ${CONFIG_FILES}; do
        install -Dm 0644 ${WORKDIR}/$i ${D}${sysconfdir}/systemd/network/$i
    done

    install -Dm 644 ${WORKDIR}/wpa_supplicant-interface@.service ${D}${systemd_unitdir}/system/wpa_supplicant-interface@.service
    if [ -e ${WORKDIR}/wpa_supplicant-common.conf ]; then
        install -d ${D}${sysconfdir}/wpa_supplicant
        install -m 0644 ${WORKDIR}/wpa_supplicant-common.conf ${D}${sysconfdir}/wpa_supplicant/wpa_supplicant-common.conf
        for i in ${WIRELESS}; do
            ln -s wpa_supplicant-common.conf ${D}${sysconfdir}/wpa_supplicant/wpa_supplicant-$i.conf

            install -d ${D}${sysconfdir}/systemd/system/multi-user.target.wants
            ln -s ${systemd_unitdir}/system/wpa_supplicant-interface@.service ${D}${sysconfdir}/systemd/system/multi-user.target.wants/wpa_supplicant-interface@$i.service
        done
    fi
}

