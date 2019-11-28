SUMMARY = "mjpg-streamer systemd unit"
LICENSE = "MIT"

inherit systemd

SYSTEMD_SERVICE_${PN} = "mjpg-streamer.service"

do_install() {
    install -d ${D}${systemd_unitdir}/system
    cat > ${D}${systemd_unitdir}/system/mjpg-streamer.service <<- __EOF
[Unit]
Description=A server for streaming Motion-JPEG from a video capture device
After=network.target

[Service]
ExecStart=/usr/bin/mjpg_streamer -i "/usr/lib/mjpg-streamer/input_raspicam.so -x 1920 -y 1080 -fps 4" -o "/usr/lib/mjpg-streamer/output_http.so"

[Install]
WantedBy=multi-user.target
__EOF
}

COMPATIBLE_MACHINE = "^$"
COMPATIBLE_MACHINE_rpi = ".*"

PACKAGE_ARCH = "${MACHINE_ARCH}"
