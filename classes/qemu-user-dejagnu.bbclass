inherit qemu
inherit dejagnu

# using qemu-native for qemu-* linux-user execution
DEPENDS += "qemu-native"

def qemu_user_run_args(d):
    qemu_binary = qemu_target_binary(d)
    qemu_binary = bb.utils.which(d.getVar("PATH"), qemu_binary)

    if qemu_binary is None:
        raise Exception("Missing binary")

    args = [qemu_binary]
    args += (d.getVar("QEMU_OPTIONS") or "").split()
    args += ["-L", d.getVar("STAGING_DIR_HOST")]
    #args += ["-E", "LD_DEBUG=all"]

    libpaths = [d.getVar("libdir"), d.getVar("base_libdir")]
    args += ["-E", "LD_LIBRARY_PATH=%s" % ":".join(libpaths)]

    return args

def generate_qemu_linux_user_config(d):
    args = qemu_user_run_args(d)
    content = []

    content.append('load_generic_config "sim"')
    content.append('load_base_board_description "basic-sim"')
    content.append('process_multilib_options ""')

    # qemu args
    content.append('set_board_info is_simulator 1')
    content.append('set_board_info sim "%s"' % args[0])
    content.append('set_board_info sim,options "%s"' % " ".join(args[1:]))

    # target build/test config
    content.append('set_board_info target_install {%s}' % d.getVar("TARGET_SYS"))
    content.append('set_board_info ldscript ""')
    #content.append('set_board_info needs_status_wrapper 1') # qemu-linux-user return codes work, and abort works fine
    content.append('set_board_info gcc,stack_size 16834')
    content.append('set_board_info gdb,nosignals 1')
    content.append('set_board_info gcc,timeout 60')

    return "\n".join(content)

do_generate_dejagnu[dirs] += "${DEJAGNU_DIR}"
addtask do_generate_dejagnu after do_configure
python do_generate_dejagnu () {
    # write out target qemu board config
    with open(os.path.join(d.getVar("DEJAGNU_DIR"), "qemu-linux-user.exp"), "w") as f:
        f.write(generate_qemu_linux_user_config(d))

    # generate .exp for qemu user
    with open(os.path.join(d.getVar("DEJAGNU_DIR"), "site.exp"), "w") as f:
        f.write("lappend boards_dir %s" % d.getVar("DEJAGNU_DIR"))
}

