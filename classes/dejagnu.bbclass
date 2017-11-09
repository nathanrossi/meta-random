DEPENDS += "${@bb.utils.contains('BBFILE_COLLECTIONS', 'openembedded-layer', 'dejagnu-native expect-native', '', d)}"

# don't clean up the build, so that check can be run
RM_WORK_EXCLUDE += "${PN}"

DEJAGNU_DIR ?= "${WORKDIR}/dejagnu"
export DEJAGNU = "${DEJAGNU_DIR}/site.exp"
DEJAGNU_TARGET ??= "qemu-linux-user"

def dejagnu_find_all_results(basepath, d):
    builddir = basepath or d.getVar("B")
    results = []
    for root, dirs, files in os.walk(builddir):
        for f in files:
            if f.endswith(".sum"):
                results.append(os.path.join(root, f))
    return results

def dejagnu_parse_results(sumfile):
    import re
    passes, fails = 0, 0
    upasses, xfails = 0, 0
    untested, unresolved, unsupported = 0, 0, 0
    errors = 0
    with open(sumfile, "r") as f:
        for i in f:
            if i.startswith("PASS:"):
                passes += 1
            elif i.startswith("XPASS:"):
                upasses += 1
            elif i.startswith("FAIL:"):
                fails += 1
            elif i.startswith("XFAIL:"):
                xfails += 1
            elif i.startswith("UNTESTED:"):
                untested += 1
            elif i.startswith("UNRESOLVED:"):
                unresolved += 1
            elif i.startswith("UNSUPPORTED:"):
                unsupported += 1
            elif i.startswith("ERROR:"):
                errors += 1
    return passes, fails, upasses, xfails, untested, unresolved, unsupported, errors

def dejagnu_write_report(basepath, d):
    for i in dejagnu_find_all_results(basepath, d):
        passes, fails, upasses, xfails, untested, unresolved, unsupported, errors = dejagnu_parse_results(i)
        bb.warn("Test Run - {0}".format(os.path.basename(i)))
        if passes > 0:
            bb.warn("  pass = {0}".format(passes))
        if fails > 0:
            bb.warn("  fail = {0}".format(fails))
        if upasses > 0:
            bb.warn("  pass (unexpected) = {0}".format(upasses))
        if xfails > 0:
            bb.warn("  fail (expected) = {0}".format(xfails))
        if untested > 0:
            bb.warn("  untested = {0}".format(untested))
        if unresolved > 0:
            bb.warn("  unresolved = {0}".format(unresolved))
        if unsupported > 0:
            bb.warn("  unsupported = {0}".format(unsupported))
        if errors > 0:
            bb.warn("  errors = {0}".format(errors))

def dejagnu_cross_env(d):
    env = os.environ.copy()
    env["CC_FOR_TARGET"] = d.expand("${CC}")
    env["GCC_FOR_TARGET"] = d.expand("${CC}")
    env["CXX_FOR_TARGET"] = d.expand("${CXX}")
    env["AS_FOR_TARGET"] = d.expand("${HOST_PREFIX}as")
    env["LD_FOR_TARGET"] = d.expand("${HOST_PREFIX}ld")
    env["NM_FOR_TARGET"] = d.expand("${HOST_PREFIX}nm")
    env["AR_FOR_TARGET"] = d.expand("${HOST_PREFIX}ar")
    env["GFORTRAN_FOR_TARGET"] = d.expand("gfortran")
    env["RANLIB_FOR_TARGET"] = d.expand("${HOST_PREFIX}ranlib")
    return env

def dejagnu_gnu_site_exp(testsuite, d):
    content = []
    content.append("set objdir \"{0}\"".format(os.path.join(d.expand("${B}"), testsuite, "testsuite")))
    content.append("set srcdir \"{0}\"".format(os.path.join(d.expand("${S}"), testsuite, "testsuite")))

    content.append("set host_triplet {0}".format(d.getVar("BUILD_SYS")))
    content.append("set host_alias {0}".format(d.getVar("BUILD_SYS")))
    content.append("set build_triplet {0}".format(d.getVar("BUILD_SYS")))
    content.append("set build_alias {0}".format(d.getVar("BUILD_SYS")))
    content.append("set target_triplet {0}".format(d.getVar("TARGET_SYS")))
    content.append("set target_alias {0}".format(d.getVar("TARGET_SYS")))

    return content

def dejagnu_run_testsuite(testsuite, tools, siteexp, d, board = True, cwd = None):
    import subprocess
    for i in tools:
        builddir = os.path.join(d.expand("${B}"), testsuite)
        workdir = os.path.join(builddir, "testsuite", i)
        if not os.path.exists(workdir):
            os.makedirs(workdir)

        if siteexp is not None and callable(siteexp):
            # create site.exp
            with open(os.path.join(workdir, "site.exp"), "w") as f:
                f.write("\n".join(siteexp(testsuite, d)))

        srcdir = os.path.join(d.expand("${S}"), testsuite, "testsuite")
        rundir = os.path.abspath(workdir if cwd is None else os.path.join(workdir, cwd))

        targetboard = d.getVar("DEJAGNU_TARGET") or None

        # run dejagnu in target directory
        r = subprocess.run(
            ["runtest"] + \
                ["--tool", i] + \
                ["--srcdir", srcdir] + \
                (["--target_board={0}".format(targetboard)] if targetboard and board else []),
            cwd = rundir,
            env = dejagnu_cross_env(d))
        #if r.returncode != 0:
            #bb.fatal("dejagnu runtest failed for '{0}'".format(i))
        bb.note("dejagnu runtest completed for '{0}'".format(i))

        # display report
        dejagnu_write_report(builddir, d)

def dejagnu_run_make_testsuite(testsuite, target, d, args = None, board = True):
    import subprocess
    builddir = os.path.join(d.expand("${B}"), testsuite)

    targetboard = d.getVar("DEJAGNU_TARGET") or None
    runtestflags = (["RUNTESTFLAGS=\"--target_board={0}\"".format(targetboard)] if targetboard and board else []),

    # run dejagnu in target directory
    r = subprocess.run(
        [i for i in d.expand("${MAKE} ${EXTRA_OEMAKE}").split(" ") if len(i) != 0] + \
            [target] + \
            runtestflags + \
            ([] if args is None else args),
        cwd = d.expand("${B}"),
        env = dejagnu_cross_env(d))
    #if r.returncode != 0:
        #bb.fatal("dejagnu runtest failed for '{0}'".format(i))
    bb.note("dejagnu runtest completed for '{0}'".format(testsuite))

    # display report
    dejagnu_write_report(builddir, d)

do_check[dirs] += "${B}"
do_check[nostamp] = "1"
python do_check() {
}
addtask do_check after do_compile do_install
