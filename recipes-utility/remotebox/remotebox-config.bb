SUMMARY = "remotebox system configuration"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COREBASE}/meta/COPYING.MIT;md5=3da9cfbcb788c80a0384361b4de20420"

inherit systemd
inherit update-alternatives

SYSTEMD_SERVICE:${PN} += "media-mmc.mount media-mmc.automount"
SYSTEMD_SERVICE:${PN} += "home-root-overlay.service"

do_install() {
    install -d ${D}${systemd_unitdir}/network
    install -d ${D}${systemd_unitdir}/system

    # force switch switch up
    cat > ${D}${systemd_unitdir}/network/eth2.network <<- __EOF
[Match]
Name=eth2

[Network]
LocalLinkAddressing=yes
__EOF
    # bridge, fixed MAC
    cat > ${D}${systemd_unitdir}/network/br0.netdev <<- __EOF
[NetDev]
Name=br0
Kind=bridge
MACAddress=00:08:a2:11:dc:bb
__EOF
    cat > ${D}${systemd_unitdir}/network/br0.network <<- __EOF
[Match]
Name=br0

[Network]
DHCP=yes

[DHCPv4]
ClientIdentifier=mac
__EOF
    cat > ${D}${systemd_unitdir}/network/br0-binds.network <<- __EOF
[Match]
Name=eth0
Name=eth1
Name=lan1
Name=lan2
Name=lan3
Name=lan4

[Network]
Bridge=br0
__EOF

    # automounts for /media/mmc
    cat > ${D}${systemd_unitdir}/system/media-mmc.mount <<- __EOF
[Mount]
What=/dev/mmcblk0p1
Where=/media/mmc

[Install]
WantedBy=local-fs.target
__EOF
    cat > ${D}${systemd_unitdir}/system/media-mmc.automount <<- __EOF
[Automount]
Where=/media/mmc

[Install]
WantedBy=local-fs.target
__EOF

    # setup the root home directory as an overlay using /media/mmc
    cat > ${D}${systemd_unitdir}/system/home-root-overlay.service <<- __EOF
[Unit]
After=mmc.mount

[Service]
Type=oneshot
ExecStart=/bin/mkdir -p /media/mmc/home /media/mmc/.home-workdir
ExecStart=/bin/mount -t overlay overlay /home/root -o lowerdir=/home/root,upperdir=/media/mmc/home,workdir=/media/mmc/.home-workdir

[Install]
WantedBy=local-fs.target
__EOF

    # Create this path in the read-only rootfs
    mkdir -p ${D}/media/mmc

    # install ssh public auth keys
    install -d -m0700 ${D}/home/root/.ssh
    cat ~/.ssh/authorized_keys | grep "local-hopping-key" > ${WORKDIR}/authorized_keys
    install -m0600 ${WORKDIR}/authorized_keys ${D}/home/root/.ssh/authorized_keys
    # install github.com to known_hosts
    cat > ${WORKDIR}/known_hosts <<- __EOF
|1|+0h5iwYbYtWg86o/dIZmrnihmPk=|KR1AJP3kQ7jU+oDZz9UXdztsH7A= ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEAq2A7hRGmdnm9tUDbO9IDSwBK6TbQa+PXYPCPy6rbTrTtw7PHkccKrpp0yVhp5HdEIcKr6pLlVDBfOLX9QUsyCOV0wzfjIJNlGEYsdlLJizHhbn2mUjvSAHQqZETYP81eFzLQNnPHt4EVVUh7VfDESU84KezmD5QlWpXLmvU31/yMf+Se8xhHTvKSCZIFImWwoG6mbUoWf9nzpIoaSjB+weqqUUmpaaasXVal72J+UX2B+2RPW3RcT0eOzQgqlJL3RKrTJvdsjE3JEAvGq3lGHSZXy28G3skua2SmVi/w4yCE6gbODqnTWlg7+wC604ydGXA8VJiS5ap43JXiUFFAaQ==
|1|9GqWJ1PYBvSCr1nkKFRhlQBnex0=|IjTnHMxPzeXSXEJ+ty8uRBpHKAU= ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBEmKSENjQEezOmxkZMy7opKgwFB9nkt5YRrYMjNuG5N87uRgg6CLrbo5wAdT/y6v0mKV0U2w0WZ2YB/++Tpockg=
|1|5mJUPjnZXxu0e2BwWAE9e5Chrrs=|9+/91P1mFNRHHBp84KurScS4UjM= ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl
__EOF
    install -m0600 ${WORKDIR}/known_hosts ${D}/home/root/.ssh/known_hosts
}

PACKAGE_ARCH = "${MACHINE_ARCH}"

FILES:${PN} += "${systemd_unitdir}/network/"
FILES:${PN} += "${systemd_unitdir}/system/"
FILES:${PN} += "/home/root/.ssh"

QA_EMPTY_DIRS:remove = "/media"
FILES:${PN} += "/media/mmc"

