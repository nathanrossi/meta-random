inherit dejagnu

DEJAGNU_TARGETS[remote] = "remote-ssh-linux"
do_check[prefuncs] += "dejagnu_qemu_user_generate"

BUILD_TEST_HOST ??= ""

def generate_remote_ssh_linux_config(d):
    content = []

    # How to compile C programs for this board
    #set_board_info compiler /usr/bin/gcc

    content.append("set_board_info hostname {0}".format(d.getVar("BUILD_TEST_HOST")))
    content.append('set_board_info username root')
    content.append('set_board_info rsh_prog /usr/bin/ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no')
    content.append('set_board_info rcp_prog /usr/bin/scp -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no')

    return "\n".join(content)

dejagnu_remote_ssh_linux_generate[dirs] += "${DEJAGNU_DIR}"
python dejagnu_remote_ssh_linux_generate () {
    # write out target qemu board config
    with open(os.path.join(d.getVar("DEJAGNU_DIR"), "remote-ssh-linux.exp"), "w") as f:
        f.write(generate_remote_ssh_linux_config(d))

    # generate .exp for qemu user
    with open(os.path.join(d.getVar("DEJAGNU_DIR"), "site.exp"), "w") as f:
        f.write("lappend boards_dir %s" % d.getVar("DEJAGNU_DIR"))
}

