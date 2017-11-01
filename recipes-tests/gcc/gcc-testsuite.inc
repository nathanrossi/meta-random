PV = "7.2"

require recipes-devtools/gcc/gcc-${PV}.inc
require recipes-devtools/gcc/gcc-configure-common.inc

inherit dejagnu
inherit dejagnu-qemu-user

#do_fetch[noexec] = "1"
do_configure[noexec] = "1"
do_compile[noexec] = "1"
do_install[noexec] = "1"

export CC_FOR_TARGET="${CC}"
export GCC_FOR_TARGET="${CC}"
export CXX_FOR_TARGET="${CXX}"
export AS_FOR_TARGET="${HOST_PREFIX}as"
export LD_FOR_TARGET="${HOST_PREFIX}ld"
export NM_FOR_TARGET="${HOST_PREFIX}nm"
export AR_FOR_TARGET="${HOST_PREFIX}ar"
export GFORTRAN_FOR_TARGET="gfortran"
export RANLIB_FOR_TARGET="${HOST_PREFIX}ranlib"

do_check[nostamp] = "1"
do_check[prefuncs] += "dejagnu_qemu_user_generate"

def dejagnu_gcc_run_testsuite(testsuite, tools, siteexp, d):
    import subprocess
    for i in tools:
        workdir = os.path.join(d.expand("${B}"), testsuite, "testsuite", i)
        if not os.path.exists(workdir):
            os.makedirs(workdir)

        if siteexp is not None and callable(siteexp):
            # create site.exp
            with open(os.path.join(workdir, "site.exp"), "w") as f:
                f.write("\n".join(siteexp(testsuite, d)))

        # run dejagnu in target directory
        r = subprocess.run(
            ["runtest", "--tool", i, "--target_board=qemu-linux-user"],
            cwd = workdir)
        #if r.returncode != 0:
            #bb.fatal("dejagnu runtest failed for '{0}'".format(i))
        bb.note("dejagnu runtest completed for '{0}'".format(i))

        # display report
        dejagnu_write_report(workdir, d)

python do_check() {
}
addtask do_check after do_compile do_install

