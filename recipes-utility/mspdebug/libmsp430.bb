SECTION = "bsp"
DEPENDS = "boost hidapi"

# Mixed licenses? BSD-3-Clause/MIT?
LICENSE = "CLOSED"

SRC_URI = "https://dr-download.ti.com/software-development/driver-or-library/MD-4vnqcP1Wk4/3.15.1.1/MSPDebugStack_OS_Package_3_15_1_1.zip;subdir=source"
SRC_URI[sha256sum] = "e3a59a98c43de7a92e5814d8c3304026165e6d2551e60acaca1f08c6b1a4bac8"

S = "${WORKDIR}/source"

do_configure() {
    # fix up include paths
    sed -i 's#<hidapi.h>#<hidapi/hidapi.h>#g' \
        ${S}/ThirdParty/BSL430_DLL/BSL430_DLL/Physical_Interfaces/MSPBSL_PhysicalInterfaceUSB.h \
        ${S}/DLL430_v3/src/TI/DLL430/HidUpdateManager.cpp

    # link with -lhidapi-libusb
    sed -i 's#..LIBTHIRD./hid-libusb.o#-lhidapi-libusb#g' \
        ${S}/Makefile
}

do_compile() {
    # inject LDFLAGS via BDYNAMIC
    oe_runmake \
        CXX="${CXX}" \
        CXXFLAGS="${CXXFLAGS} -fPIC" \
        BDYNAMIC="-Wl,-Bdynamic ${LDFLAGS}"
}

do_install() {
    install -d ${D}${libdir}
    install -m 755 ${S}/libmsp430.so ${D}${libdir}/
}

FILES_SOLIBSDEV = ""
FILES:${PN} += "${libdir}/libmsp430.so"

# hidapi depends on libudev
#BBCLASSEXTEND = "native nativesdk"
