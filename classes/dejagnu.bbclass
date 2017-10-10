DEPENDS += "${@bb.utils.contains('BBFILE_COLLECTIONS', 'openembedded-layer', 'dejagnu-native expect-native', '', d)}"

DEJAGNU_DIR ?= "${WORKDIR}/dejagnu"

addtask do_check after do_configure do_compile
do_check[dirs] += "${B}"
do_check[nostamp] = "1"
do_check () {
	export DEJAGNU="${DEJAGNU_DIR}/site.exp"
	oe_runmake -k check V=1
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

addtask do_check_report
python do_check_report () {
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

