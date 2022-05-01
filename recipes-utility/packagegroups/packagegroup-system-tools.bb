SUMMARY = "System tools for debugging, disks, network, etc."

inherit packagegroup

RDEPENDS:${PN} += " \
    openssh \
    openssh-sftp-server \
    openssh-sftp \
    \
    iputils \
    iproute2 \
    tcpdump \
    ethtool \
    \
    usbutils \
    pciutils \
    i2c-tools \
    hdparm \
    \
    util-linux \
    e2fsprogs \
    dosfstools \
    fuse-exfat \
    exfat-utils \
    \
    strace \
    file \
    \
    python3 \
    "

