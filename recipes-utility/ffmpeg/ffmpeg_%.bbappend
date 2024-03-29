# add override to conditionally change source
OVERRIDES:prepend = "${@bb.utils.contains("PACKAGECONFIG", "omx-rpi", "omx-rpi:", "", d)}"

PACKAGECONFIG[omx-rpi] = "--enable-omx-rpi,,virtual/libomxil,userland"

# handle omx headers being in IL subdir
CFLAGS:append:omx-rpi = " -isystem =${includedir}/IL"

do_configure:prepend:omx-rpi() {
    # make use of system dirs include of /opt/vc/* dirs
    sed -i 's#/opt/vc/lib#${libdir}#g' ${S}/libavcodec/omx.c
}

