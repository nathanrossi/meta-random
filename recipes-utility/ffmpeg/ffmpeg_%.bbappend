# add override to conditionally change source
OVERRIDES_prepend = "${@bb.utils.contains("PACKAGECONFIG", "omx-rpi", "omx-rpi:", "", d)}"

PACKAGECONFIG[omx-rpi] = "--enable-omx-rpi,,virtual/libomxil,userland"

# handle omx headers being in IL subdir
CFLAGS_append_omx-rpi = " -isystem =${includedir}/IL"

do_configure_prepend_omx-rpi() {
    # make use of system dirs include of /opt/vc/* dirs
    sed -i 's#/opt/vc/lib#${libdir}#g' ${S}/libavcodec/omx.c
}

