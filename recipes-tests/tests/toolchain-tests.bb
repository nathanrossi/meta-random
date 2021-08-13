DESCRIPTION = "Toolchain test programs"
LICENSE = "MIT"

LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/MIT;md5=0835ade698e0bcf8506ecda2f7b4f302"
SRC_URI = "${@find_sources(d)}"

TARGET_LDFLAGS += "-lpthread -lgcc_s"

def find_sources(d):
    srcs = []
    for path in d.getVar("FILESPATH").split(":"):
        if not os.path.isdir(path):
            continue
        for i in os.listdir(path):
            if os.path.splitext(i)[1] in [".c", ".cpp", ".h"]:
                srcs.append(i)
    return " ".join("file://{0}".format(i) for i in srcs)

def compiler(d, args, cxx = False):
    cmd = d.getVar("CC" if not cxx else "CXX").split(" ")
    cmd += d.getVar("TARGET_CFLAGS" if not cxx else "TARGET_CXXFLAGS").split(" ")
    cmd += args
    cmd += d.getVar("TARGET_LDFLAGS").split(" ")
    return [i for i in cmd if i]

python do_compile() {
    import subprocess
    for i in os.listdir(d.getVar("WORKDIR")):
        base, ext = os.path.splitext(i)
        if ext in [".c", ".cpp"]:
            for opt in ["-O2", "-Os", "-O1", "-O0"]:
                # output
                cmd = ["-o", os.path.join(d.getVar("B"), base + opt.lower())]
                cmd += [os.path.join(d.getVar("WORKDIR"), i)]
                cmd += [opt]

                r = subprocess.run(compiler(d, cmd, cxx = (ext == ".cpp")), cwd = d.getVar("B"))
                if r.returncode != 0:
                    bb.fatal("Failed to compile '{0}'".format(base))
}

do_install() {
	install -d ${D}${bindir}
	for i in ${B}/*; do
		install -m 0755 $i ${D}${bindir}/$(basename $i)
	done
}

FILES:${PN} += "${bindir}"

