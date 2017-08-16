#!/bin/bash

#DEPLOY=/home/nathan/build/tmp-glibc/deploy/images/zybo-zynq7
DEPLOY=/home/nathan/build/tmp-glibc/deploy/images/zc706-zynq7

if [ ! -f $DEPLOY/u-boot-spl.bin ]; then
	dd if=$DEPLOY/boot.bin of=$DEPLOY/u-boot-spl.bin bs=1 skip=2240
fi
if [ ! -f $DEPLOY/u-boot-dtb.bin ]; then
	dd if=$DEPLOY/u-boot-dtb.img of=$DEPLOY/u-boot-dtb.bin skip=64 bs=1
fi

openocd \
	-f digilent-hs3-smt.cfg \
	-f zc706.cfg \
	-c "adapter_khz 2000" \
	-c "init" \
	-c "halt" \
	-c "reg pc" \
	-c "load_image $DEPLOY/u-boot-spl.bin 0x0" \
	-c "reg pc 0x0" \
	-c "resume" \
	-c "sleep 4000" \
	-c "halt" \
	-c "load_image $DEPLOY/u-boot-dtb.bin 0x4000000" \
	-c "reg pc 0x4000000" \
	-c "resume" \
	-c "shutdown"

