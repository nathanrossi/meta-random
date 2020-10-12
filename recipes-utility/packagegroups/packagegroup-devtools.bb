SUMMARY = "devtools packagegroup"

inherit packagegroup

RDEPENDS_${PN} = " \
    coreutils findutils \
    tar xz unzip \
    p7zip \
    rsync \
    \
    fish-shell \
    \
    git \
    \
    ncurses-terminfo \
    ncurses-terminfo-base \
    vim \
    vim-common \
    vim-syntax \
    vim-tools \
    \
    python3 python3-modules \
    python3-requests \
    python3-pytz \
    python3-tzlocal \
    \
    tmux \
    \
    dtc \
    dnsmasq \
    \
    "

BBCLASSEXTEND += "nativesdk"
