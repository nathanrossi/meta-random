
inherit qemu-user-dejagnu

addtask do_check after do_generate_dejagnu do_configure do_compile
do_check[dirs] += "${B}"
do_check[nostamp] = "1"
do_check () {
	export DEJAGNU="${DEJAGNU_DIR}/site.exp"
	oe_runmake -k check-gcc V=1 RUNTESTFLAGS="--target_board=qemu-linux-user"
	#oe_runmake -k check-gcc RUNTESTFLAGS="execute.exp=pr68390.c --target_board=qemu-linux-user"
}

