
python do_generate_config() {
    def config(name, val):
        cfgname = "CONFIG_{}".format(name)
        cfgcomment = "# " + cfgname
        data = ""
        with open(d.expand("${B}/.config"), "r") as source:
            matched = False
            for l in source:
                if l.startswith(cfgname + "=") or l.startswith(cfgcomment + " "):
                    data += "CONFIG_{}={}\n".format(name, val)
                    matched = True
                else:
                    data += l
            if not matched: # append if not set
                data += "CONFIG_{}={}\n".format(name, val)

        with open(d.expand("${B}/.config"), "w") as dest:
            dest.write(data)

    with open(d.expand("${B}/.config"), "w") as f:
        with open(d.expand("${S}/arch/${ARCH}/configs/${KBUILD_DEFCONFIG}"), "r") as src:
            f.write(src.read())
}
addtask generate_config before do_configure after do_unpack

