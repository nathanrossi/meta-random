require recipes-devtools/gcc/gcc-${PV}.inc
require recipes-devtools/gcc/gcc-configure-common.inc
require gcc-testsuite.inc

# dont execute as build tasks normally
do_configure[noexec] = "1"
do_compile[noexec] = "1"
do_install[noexec] = "1"

# make sure compiler/libc are available
INHIBIT_DEFAULT_DEPS = ""

# have the build directory available
do_gcc_check[depends] += "${COMPILERDEP}"
do_gcc_check[prefuncs] += "extract_stashed_builddir_sysroot"

python extract_stashed_builddir_sysroot () {
    src = d.expand("${COMPONENTS_DIR}/${BUILD_ARCH}/gcc-stashed-builddir${COMPILERINITIAL}-${TARGET_SYS}")
    dest = d.getVar("B")
    recipesysroot = d.getVar("RECIPE_SYSROOT")
    recipesysrootnative = d.getVar("RECIPE_SYSROOT_NATIVE")
    oe.path.copyhardlinktree(src, dest)
    staging_processfixme([src + "/fixmepath"], dest, recipesysroot, recipesysrootnative, d)
}
