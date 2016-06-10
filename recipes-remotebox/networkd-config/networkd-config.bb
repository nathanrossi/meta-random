DESCRIPTION = "Generate Config files for networkd managed interfaces"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COREBASE}/LICENSE;md5=4d92cd373abda3937c2bc47fbc49d690"

PACKAGE_ARCH = "${MACHINE_ARCH}"

DEPENDS = "wpa-supplicant-native"
RDEPENDS_${PN} = "wpa-supplicant"

CONFIG_INTERFACES ??= ""

inherit systemd

WLAN_SYSTEMD_UNITS ?= ""
SYSTEMD_AUTO_ENABLE ?= "enable"

python () {
    for i in (d.getVar("CONFIG_INTERFACES", True) or "").split():
        flags = (d.getVarFlag("CONFIG_INTERFACES", i) or "").split(",")

        addrsubnet = flags[0] if len(flags) >= 1 else "dhcp"
        gateway = flags[1] if len(flags) >= 2 else "dhcp"
        dns = flags[2] if len(flags) >= 3 else "dhcp"
        wifissid = flags[3] if len(flags) >= 4 else None
        wifipsk = flags[4] if len(flags) >= 5 else None

        if wifissid != None and wifipsk != None:
            d.setVar("WLAN_SYSTEMD_UNITS", "%s %s" %
                (d.getVar("WLAN_SYSTEMD_UNITS", True), "wpa_supplicant@%s.conf" % i))
}

python do_generate_configs() {
    systemd_path = os.path.join(d.getVar("S", True), "generate", "etc/systemd/network")
    wpa_path = os.path.join(d.getVar("S", True), "generate", "etc/wpa_supplicant")
    for i in (d.getVar("CONFIG_INTERFACES", True) or "").split():
        flags = (d.getVarFlag("CONFIG_INTERFACES", i) or "").split(",")

        addrsubnet = flags[0] if len(flags) >= 1 else "dhcp"
        gateway = flags[1] if len(flags) >= 2 else "dhcp"
        dns = flags[2] if len(flags) >= 3 else "dhcp"
        wifissid = flags[3] if len(flags) >= 4 else None
        wifipsk = flags[4] if len(flags) >= 5 else None

        # populate networkd conf
        cfg = "[Match]\n"
        cfg += "Name=%s\n" % i
        cfg += "\n"
        cfg += "[Network]\n"
        if addrsubnet == "dhcp" and gateway == "dhcp" and dns == "dhcp":
            cfg += "DHCP=yes\n"
        else:
            cfg += "Address=%s\n" % addrsubnet
            cfg += "Gateway=%s\n" % gateway
            cfg += "DNS=%s\n" % dns

        # write files to filesystem
        cfgfile = os.path.join(systemd_path, "%s.network" % i)
        with open(cfgfile, "w") as f:
            f.write(cfg)

        # wifi config
        if wifissid != None and wifipsk != None:
            import subprocess
            psk = subprocess.check_output(["wpa_passphrase", wifissid, wifipsk])
            cfgfile = os.path.join(wpa_path, "wpa_supplicant-%s.conf" % i)
            with open(cfgfile, "w") as f:
                f.write(psk)
}
addtask generate_configs after do_compile before do_install

do_compile() {
    mkdir -p ${S}/generate/etc/systemd/network
    mkdir -p ${S}/generate/etc/wpa_supplicant
}

do_install() {
    install -d ${D}/etc/systemd/network
    for i in $(ls ${S}/generate/etc/systemd/network); do
        install ${S}/generate/etc/systemd/network/$i ${D}/etc/systemd/network
    done

    install -d ${D}/etc/wpa_supplicant
    install -d ${D}${sysconfdir}/systemd/system/multi-user.target.wants
    for i in $(ls ${S}/generate/etc/wpa_supplicant); do
        install ${S}/generate/etc/wpa_supplicant/$i ${D}/etc/wpa_supplicant
        IFNAME=$(echo $i | sed 's/wpa_supplicant-//g' | sed 's/\.conf//g')
        ln -sf ${systemd_unitdir}/system/wpa_supplicant\@.service \
            ${D}${sysconfdir}/systemd/system/multi-user.target.wants/wpa_supplicant\@${IFNAME}.service
    done
}

FILES_${PN} += " \
        /etc/wpa_supplicant/* \
        /etc/systemd/network/* \
        ${sysconfdir}/systemd/system/multi-user.tar.get.wants/* \
        "

