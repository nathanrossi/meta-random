inherit devicetree

SRC_URI = " \
        file://bcm2835-rpi-zero-spidev.dts \
        file://bcm2837-rpi-3-b-plus-toolbox.dts \
        file://rpi-uart0-rtscts.dts \
        "

COMPATIBLE_MACHINE = "rpi"

