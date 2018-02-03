DESCRIPTION = "Native tools available locally for use outside of bitbake/oe"
LICENSE = "MIT"

RM_WORK_EXCLUDE += "${PN}"
INHIBIT_DEFAULT_DEPS = "1"
inherit nopackages
inherit native

# Selection of desired dev-tools
DEPENDS += "u-boot-mkimage-native"

COMMONBINDIR = "${TMPDIR}/devtools"

do_setup_links[dirs] += "${WORKDIR}/bin"
python do_setup_links () {
    binpath = os.path.abspath(os.path.join(d.getVar("WORKDIR"), "bin"))
    if not os.path.exists(binpath):
        os.makedirs(binpath)

    # link it into the base of the tmpdir
    commonbinpath = os.path.join(d.getVar("TMPDIR"), "devtools")
    if not os.path.lexists(commonbinpath):
        os.symlink(os.path.relpath(binpath, os.path.dirname(commonbinpath)), commonbinpath)

    # populate symlinks for all usr/bin and bin/ programs
    for bindir in ["bin", "usr/bin"]:
        bindirpath = os.path.abspath(os.path.join(d.getVar("RECIPE_SYSROOT_NATIVE"), bindir))
        if not os.path.exists(bindirpath):
            continue
        for i in os.listdir(bindirpath):
            spath = os.path.join(bindirpath, i)
            tpath = os.path.join(binpath, i)
            relpath = os.path.relpath(spath, binpath)
            # remove and relink
            if not os.path.lexists(tpath) or os.readlink(tpath) != relpath:
                if os.path.lexists(tpath):
                    os.remove(tpath)
                os.symlink(relpath, tpath)
}
addtask setup_links before do_build after do_prepare_recipe_sysroot

