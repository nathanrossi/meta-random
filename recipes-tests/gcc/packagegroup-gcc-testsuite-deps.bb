SUMMARY = "Deps for running GCC testsuite on the target"

inherit packagegroup

RDEPENDS_${PN} = " \
		libstdc++ \
		libatomic \
		libgomp \
		"

