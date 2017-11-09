inherit dejagnu

python do_check() {
    dejagnu_run_make_testsuite("binutils", "check-binutils", d, board = False)
    dejagnu_run_make_testsuite("gas", "check-gas", d, board = False)
    dejagnu_run_make_testsuite("ld", "check-ld", d, board = False)
}
