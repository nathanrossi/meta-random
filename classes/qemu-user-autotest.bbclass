
inherit qemu

python () {
    if "class-target" in (d.getVar("OVERRIDES") or "").split(":"):
        d.appendVar("DEPENDS", " qemu-native")
        bb.build.addtask("do_qemu_user_tests", None, "do_install", d)
}

# setup the sysroot so that target binaries are populated, this allows for the
# use of things like python, etc. during the test phase
SYSROOT_DIRS_append_class-target = " \
    ${bindir} \
    ${sbindir} \
    ${base_bindir} \
    ${base_sbindir} \
    ${libexecdir} \
    "

def qemu_user_find_interp_path(target, stagedir):
    interpstr = None
    with open(target, "rb") as f:
        if f.read(2) == "#!".encode():
            interpstr = f.read().splitlines()[0].decode()
        else:
            return []

    interp = None
    parts = interpstr.split()
    if len(parts) == 1:
        interp = [os.path.join(stagedir, parts[0])]
    elif len(parts) > 1 and parts[0] == "/usr/bin/env":
        interptarget = parts[1]
        if not interptarget.startswith("/"):
            # figure out the actual target binary
            paths = [os.path.join(stagedir, i) for i in ["bin", "usr/bin", "sbin", "usr/sbin"]]
            interptarget = bb.utils.which(":".join(paths), interptarget)
            if not interptarget:
                return None
        else:
            interptarget = stagedir + interptarget
        interp = [interptarget] + parts[2:]
    return interp

do_qemu_user_tests[dirs] += "${B}"
python do_qemu_user_tests () {
    import subprocess

    tests = (d.getVar("QEMU_USER_TESTS") or "").split()
    passed = 0
    for i in tests:
        target = os.path.abspath(i)
        stagedir = d.getVar("STAGING_DIR_HOST")
        args = qemu_wrapper_cmdline(d, stagedir, [
            os.path.join(stagedir, d.getVar("libdir")),
            os.path.join(stagedir, d.getVar("base_libdir")),
            ]).split()
        if d.getVar("QEMU_USER_STRACE"):
            args = ["QEMU_STRACE=1"] + args
        if d.getVar("QEMU_USER_LDDEBUG"):
            args += ["-E", "LD_DEBUG=all"]

        # qemu does not handle scripts/etc. that use a subprocess for execution
        interp = qemu_user_find_interp_path(target, stagedir)
        if interp == None:
            bb.warn("Skipping test '%s', unknown interp string in %s" % (i, target))
            continue
        else:
            bb.note("Using interp args %s" % repr(interp))
            args = args + interp

        bb.note("Running %s" % i)
        bb.note("QEMU - 'env %s'" % (" ".join(args + [target])))
        bb.note("=" * 80)
        try:
            bb.process.run(["env"] + args + [os.path.abspath(i)], stderr = None, stdout = None)
            passed += 1
        except bb.process.ExecutionError as e:
            bb.note("Failed %s (return = %d)" % (i, e.exitcode))
        bb.note("=" * 80)

    bb.warn("Ran qemu user tests, %d/%d" % (passed, len(tests)))
}

