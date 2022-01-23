SUMMARY = "System tools for debugging, disks, network, etc."

inherit packagegroup

RDEPENDS:${PN} += " \
    iputils \
    iproute2 \
    tcpdump \
    ethtool \
    \
    usbutils \
    pciutils \
    i2c-tools \
    \
    util-linux \
    e2fsprogs \
    dosfstools \
    \
    strace \
    file \
    \
    python3 \
    "

