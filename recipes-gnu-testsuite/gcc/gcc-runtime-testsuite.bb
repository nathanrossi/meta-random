require gcc-testsuite.inc

DEPENDS += "gcc-runtime"

# CXX, etc are not passed through cleanly to the test runner (it forces the use of [transform "g++"])
CFLAGS_prepend = "${TOOLCHAIN_OPTIONS} "
CXXFLAGS_prepend = "${TOOLCHAIN_OPTIONS} "
CPPFLAGS_prepend = "${TOOLCHAIN_OPTIONS} "
LDFLAGS_prepend = "${TOOLCHAIN_OPTIONS} "

def dejagnu_libstdcxx_site_exp(testsuite, d):
    content = dejagnu_gnu_site_exp(testsuite, d)
    # TODO: this directory is different on non-microblaze targets, sort that out?
    content.append("set baseline_dir \"{0}\"".format(os.path.join(d.expand("${S}"), testsuite, "config", "abi", "post/")))
    content.append("set baseline_subdir_switch \"--print-multi-directory\"")

    content.append("set libiconv \"\"")

    return content

def dejagnu_libatomic_site_exp(testsuite, d):
    content = dejagnu_libstdcxx_site_exp(testsuite, d)
    content.append("set GCC_UNDER_TEST \"{0}\"".format(d.getVar("CC")))
    return content

def dejagnu_libgomp_site_exp(testsuite, d):
    content = dejagnu_libstdcxx_site_exp(testsuite, d)
    content.append("set GCC_UNDER_TEST \"{0}\"".format(d.getVar("CC")))
    content.append("set GXX_UNDER_TEST \"{0}\"".format(d.getVar("CXX")))
    content.append("set cuda_driver_include \"\"")
    content.append("set cuda_driver_lib \"\"")
    content.append("set hsa_runtime_lib \"\"")
    content.append("set offload_targets \"\"")
    content.append("set offload_additional_options \"\"")
    content.append("set offload_additional_lib_paths \"\"")
    return content

python do_check() {
    dejagnu_run_testsuite("libstdc++-v3", ["libstdc++"], dejagnu_libstdcxx_site_exp, d)
    dejagnu_run_testsuite("libatomic", ["libatomic"], dejagnu_libatomic_site_exp, d)
    dejagnu_run_testsuite("libgomp", ["libgomp"], dejagnu_libgomp_site_exp, d)
}

