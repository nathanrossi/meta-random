DEPENDS += "${@bb.utils.contains('BBFILE_COLLECTIONS', 'openembedded-layer', 'dejagnu-native expect-native', '', d)}"

DEJAGNU_DIR ?= "${WORKDIR}/dejagnu"
DEJAGNU_MAKEFLAGS ?= ""

python () {
    for i in (d.getVarFlags("DEJAGNU_TARGETS") or []):
        target_task = "do_check_{0}".format(i)
        # create the task
        bb.build.addtask(target_task, None, "do_configure do_compile", d)

        # setup the task dirs/funcs/etc.
        d.appendVarFlag(target_task, "dirs", " ${B}")
        d.appendVarFlag(target_task, "postfuncs", " dejagnu_check_report")
        d.setVarFlag(target_task, "nostamp", "1")

        # setup the function/task content
        d.setVarFlag(target_task, "func", 1)
        d.setVar(target_task,
            "export DEJAGNU=\"${DEJAGNU_DIR}/site.exp\"\n" +
            "oe_runmake -k check V=1 RUNTESTFLAGS=\"${{DEJAGNU_MAKEFLAGS}} --target_board={0}\"\n".format(d.getVarFlag("DEJAGNU_TARGETS", i)))
}

def dejagnu_find_all_results(d):
    builddir = d.getVar("B")
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
    untested, unsupported = 0, 0
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
            elif i.startswith("UNSUPPORTED:"):
                unsupported += 1
    return passes, fails, upasses, xfails, untested, unsupported

python dejagnu_check_report () {
    for i in dejagnu_find_all_results(d):
        passes, fails, upasses, xfails, untested, unsupported = dejagnu_parse_results(i)
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
        if unsupported > 0:
            bb.warn("  unsupported = {0}".format(unsupported))
}

