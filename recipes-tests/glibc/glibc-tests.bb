def get_filespath_extra(d, subpath, bp = None, bpn = None):
    metaroot = next((p for p in d.getVar('BBPATH').split(':') if os.path.basename(p) == 'meta'), None)
    if metaroot:
        target = os.path.join(metaroot, subpath)
        return base_set_filespath(
            ([os.path.join(target, bp)] if bp else []) +
            ([os.path.join(target, bpn)] if bpn else []) +
            ([os.path.join(target, "files")]), d) + ":"
    return ""

PV = "${@d.getVar('GLIBCVERSION').strip('%')}"
require recipes-core/glibc/glibc_${PV}.bb

# fix up src_uri
FILESPATH_prepend := "${@get_filespath_extra(d, 'recipes-core/glibc', 'glibc-%s' % d.getVar('PV'), 'glibc')}"

# strip provides
PROVIDES = ""
# get some extra depends
DEPENDS += "libgcc"
INHIBIT_DEFAULT_DEPS = ""
# remove the initial depends
DEPENDS_remove = "libgcc-initial"
DEPENDS_remove = "linux-libc-headers"
DEPENDS_remove = "virtual/${TARGET_PREFIX}libc-initial"
DEPENDS_remove = "virtual/${TARGET_PREFIX}gcc-initial"

# this build is not deployed
#do_populate_sysroot[noexec] = "1"
SYSROOT_DIRS = ""
do_install[noexec] = "1"
deltask do_build
inherit nopackages

BUILD_TEST_HOST ??= ""
BUILD_TEST_HOST_USER ??= "root"
BUILD_TEST_HOST_PORT ??= "22"

python dummy_test_wrapper_content_ssh() {
    import sys
    import os
    import subprocess

    args = ["ssh", "-p", port, "-o", "UserKnownHostsFile=/dev/null", "-o", "StrictHostKeyChecking=no", "{0}@{1}".format(user, host), "sh", "-c"]

    command = ""
    #command += "export TIMEOUTFACTOR=10000; "
    command += " ".join(["'%s'" % i.replace("'", r"'\''") for i in ["cd", os.getcwd()]]) + "; "
    command += " ".join(["'%s'" % i.replace("'", r"'\''") for i in sys.argv[1:]])
    args.append("\"%s\"" % command)

    r = subprocess.run(args)
    sys.exit(r.returncode)
}

python setup_remote_mount_ssh() {
    import subprocess
    host = d.getVar("BUILD_TEST_HOST")
    user = d.getVar("BUILD_TEST_HOST_USER")
    port = d.getVar("BUILD_TEST_HOST_PORT")
    args = ["ssh", "-p", port, "-o", "UserKnownHostsFile=/dev/null", "-o", "StrictHostKeyChecking=no", "{0}@{1}".format(user, host), "sh", "-c"]
    args += ["\"mkdir -p /mnt/storagedisk/nathan/build; mount 10.0.10.1:/mnt/storagedisk/nathan/build /mnt/storagedisk/nathan/build -o noac\""]
    r = subprocess.run(args)
}

generate_test_wrapper[dirs] += "${WORKDIR}"
generate_test_wrapper[vardeps] += "dummy_test_wrapper_content_ssh"
python generate_test_wrapper() {
    testwrapper = os.path.join(d.getVar("WORKDIR"), "check-test-wrapper")
    with open(testwrapper, "w") as f:
        f.write("%s\n" % "#!/usr/bin/env python3")
        f.write("host = \"{0}\"\n".format(d.getVar("BUILD_TEST_HOST")))
        f.write("user = \"{0}\"\n".format(d.getVar("BUILD_TEST_HOST_USER")))
        f.write("port = \"{0}\"\n".format(d.getVar("BUILD_TEST_HOST_PORT")))
        for i in d.getVar("dummy_test_wrapper_content_ssh").splitlines():
            f.write("%s\n" % i[4:])
    os.chmod(testwrapper, 0o755)
}

do_check[dirs] += "${B}"
do_check[nostamp] = "1"
do_check[prefuncs] += "setup_remote_mount_ssh generate_test_wrapper"
addtask do_check after do_compile
do_check () {
	oe_runmake test-wrapper='${WORKDIR}/check-test-wrapper' PARALLELMFLAGS="-j2" check
}

