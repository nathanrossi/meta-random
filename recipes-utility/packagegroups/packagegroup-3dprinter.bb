SUMMARY = "3D printer base system packagegroup"

PACKAGE_ARCH = "${MACHINE_ARCH}"

inherit packagegroup

RDEPENDS_${PN} = " \
		packagegroup-base \
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

RRECOMMENDS_${PN} = " \
		${MACHINE_EXTRA_RRECOMMENDS} \
		"

