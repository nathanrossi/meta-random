SUMMARY = "Deps for running GLibc testsuite on the target"

inherit packagegroup

RDEPENDS_${PN} = " \
		glibc-charmaps \
		libgcc \
		libstdc++ \
		libatomic \
		libgomp \
		python3 \
		python3-pexpect \
		nfs-utils \
		"

