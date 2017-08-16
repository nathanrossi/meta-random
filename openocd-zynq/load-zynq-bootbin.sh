#!/bin/bash

TMPBIN=$(mktemp)
dd if=$1 of=$TMPBIN bs=1 skip=2240

SCRIPT_DIR=$(readlink -f $(dirname $0))
openocd \
	-s $SCRIPT_DIR \
	-f digilent-hs1.cfg \
	-c "transport select jtag" \
	-f zybo.cfg \
	-c "adapter_khz 10000" \
	-c "init" \
	-c "zynq_restart 100" \
	-c "load_image $TMPBIN" \
	-c "reg pc 0x0" \
	-c "echo \"--- Booting boot.bin ---\"" \
	-c "resume" \
	-c "sleep 1000" \
	-c "halt" \
	-c "echo \"--- Loading fitImage ---\"" \
	-c "load_image $2 0xf000000" \
	-c "echo \"--- Reseting and reloading boot.bin ---\"" \
	-c "zynq_restart 100" \
	-c "load_image $TMPBIN" \
	-c "reg pc 0x0" \
	-c "echo \"--- Rebooting boot.bin ---\"" \
	-c "resume" \
	-c "shutdown" \
	#-c "sleep 10000" \
	#-c "resume" \

