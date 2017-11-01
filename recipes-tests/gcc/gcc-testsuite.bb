require gcc-testsuite.inc

# check-c check-c++ check-lto
# check-g++

def dejagnu_gcc_generate_site_exp(testsuite, d):
    content = []
    content.append("set rootme \"{0}\"".format(os.path.join(d.expand("${B}"), testsuite, "testsuite")))
    content.append("set srcdir \"{0}\"".format(os.path.join(d.expand("${S}"), testsuite, "testsuite")))

    content.append("set host_triplet {0}".format(d.getVar("BUILD_SYS")))
    content.append("set host_alias {0}".format(d.getVar("BUILD_SYS")))
    content.append("set build_triplet {0}".format(d.getVar("BUILD_SYS")))
    content.append("set build_alias {0}".format(d.getVar("BUILD_SYS")))
    content.append("set target_triplet {0}".format(d.getVar("TARGET_SYS")))
    content.append("set target_alias {0}".format(d.getVar("TARGET_SYS")))

    content.append("set libiconv \"\"")

    content.append("set CFLAGS \"\"")
    content.append("set CXXFLAGS \"\"")
    content.append("set HOSTCC \"{0}\"".format(d.getVar("BUILD_CC")))
    content.append("set HOSTCFLAGS \"{0}\"".format(d.getVar("BUILD_CFLAGS")))
    content.append("set TEST_ALWAYS_FLAGS \"{0}\"".format(d.getVar("TOOLCHAIN_OPTIONS")))

    content.append("set TESTING_IN_BUILD_TREE 1")
    content.append("set HAVE_LIBSTDCXX_V3 1")

    content.append("set ENABLE_PLUGIN 1")
    content.append("set PLUGINCC \"{0}\"".format(d.getVar("BUILD_CXX")))
    content.append("set PLUGINCFLAGS \"{0}\"".format(d.getVar("BUILD_CXXFLAGS")))
    content.append("set GMPINC \"{0}\"".format(d.getVar("BUILD_CPPFLAGS")))

    return content

python do_check() {
    dejagnu_gcc_run_testsuite("gcc", ["gcc", "g++"], dejagnu_gcc_generate_site_exp, d)
}

