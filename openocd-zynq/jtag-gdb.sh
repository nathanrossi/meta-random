#!/bin/bash

SCRIPT_DIR=$(readlink -f $(dirname $0))
openocd \
	-s $SCRIPT_DIR \
	-f digilent-hs1.cfg \
	-c "transport select jtag" \
	-f zybo.cfg \
	-c "adapter_khz 2000" \
	-c "init"

