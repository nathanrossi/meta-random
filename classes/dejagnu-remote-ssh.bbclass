inherit dejagnu

do_check[prefuncs] += "dejagnu_remote_ssh_linux_generate"

BUILD_TEST_HOST ??= ""
BUILD_TEST_HOST_USER ??= "root"
BUILD_TEST_HOST_PORT ??= "22"

def generate_remote_ssh_linux_config(d):
    content = []

    # How to compile C programs for this board
    #set_board_info compiler /usr/bin/gcc

    content.append('load_generic_config "unix"')
    #content.append('load_base_board_description "basic-sim"')

    content.append("set_board_info hostname {0}".format(d.getVar("BUILD_TEST_HOST")))
    content.append("set_board_info username {0}".format(d.getVar("BUILD_TEST_HOST_USER")))

    port = d.getVar("BUILD_TEST_HOST_PORT")
    content.append("set_board_info rsh_prog \"/usr/bin/ssh -p {0} -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no\"".format(port))
    content.append("set_board_info rcp_prog \"/usr/bin/scp -P {0} -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no\"".format(port))

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

