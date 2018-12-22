PV = "8.2"

require recipes-devtools/gcc/gcc-${PV}.inc
require recipes-devtools/gcc/gcc-configure-common.inc

inherit dejagnu
inherit dejagnu-qemu-user
inherit dejagnu-remote-ssh

#do_fetch[noexec] = "1"
do_configure[noexec] = "1"
do_compile[noexec] = "1"
do_install[noexec] = "1"

