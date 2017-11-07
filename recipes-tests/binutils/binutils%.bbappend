inherit dejagnu

python do_check() {
    dejagnu_run_make_testsuite("binutils", "check-binutils", d)
    dejagnu_run_make_testsuite("gas", "check-gas", d)
    dejagnu_run_make_testsuite("ld", "check-ld", d)
}
