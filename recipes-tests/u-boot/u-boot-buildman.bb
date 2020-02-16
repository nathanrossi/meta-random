require recipes-bsp/u-boot/u-boot_2020.01.bb

SRC_URI = "git://nathan-x1.home.rossihq.com/home/nathan/dev/u-boot;protocol=ssh;branch=master"
SRCREV = "${AUTOREV}"

do_configure[noexec] = "1"
do_install[noexec] = "1"
do_deploy[noexec] = "1"

inherit nopackages

INHIBIT_DEFAULT_DEPS = "1"

# needed for fetching toolchains
DEPENDS += "ca-certificates-native"
DEPENDS += "zip-native"

DEPENDS += "lzop-native srecord-native"

# for iasl - x86
DEPENDS += "acpica-native"

BUILD_CFLAGS += "-I${STAGING_DIR_NATIVE}/usr/include/python3.8"

do_compile() {
    export GIT_SSL_CAINFO="${STAGING_DIR_NATIVE}${sysconfdir}/ssl/certs/ca-certificates.crt"
    export OPENSSL_CONF="${STAGING_DIR_NATIVE}${sysconfdir}/ssl/openssl.cnf"
    export SSL_CERT_DIR="${STAGING_DIR_NATIVE}${sysconfdir}/ssl/certs"

    cp ${STAGING_DIR_NATIVE}${libdir}/python-sysconfigdata/_sysconfigdata.py \
        ${STAGING_DIR_NATIVE}${libdir}/python3.8/

    # manually provide 'cc', no easy way to pass HOSTCC to buildman
    cat << EOF > ${STAGING_DIR_NATIVE}/usr/bin/cc
#!/bin/sh
${BUILD_CC} ${BUILD_CFLAGS} ${BUILD_LDFLAGS} \$*
EOF
    chmod +x ${STAGING_DIR_NATIVE}/usr/bin/cc

    cd ${S}

    #./tools/buildman/buildman --fetch-arch arm
    # --force-build \
    ./tools/buildman/buildman \
        --branch=HEAD \
        siemens samsung \
        am33xx \
        --detail --verbose --show_errors \
        --count=1 \
        --output-dir=${B}
}

UBOOT_MACHINE = "none"
COMPATIBLE_MACHINE = ".*"
