SUMMARY = "3D printer base system packagegroup"

PACKAGE_ARCH = "${MACHINE_ARCH}"

inherit packagegroup

RDEPENDS:${PN} = " \
		packagegroup-base \
		\
		kernel-modules \
		\
		coreutils findutils \
		tar xz unzip \
		tmux \
		ncurses-terminfo \
		ncurses-terminfo-base \
		\
		ffmpeg \
		v4l-utils \
		\
		printrun \
		"

RRECOMMENDS:${PN} = " \
		${MACHINE_EXTRA_RRECOMMENDS} \
		"

