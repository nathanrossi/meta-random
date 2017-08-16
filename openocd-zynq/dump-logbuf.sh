#!/bin/bash

PARAMS=$(readelf -a $1 | grep __log_buf | sed -r "s/.*?: ([a-f0-9]*) (0x[0-9]*|[0-9]*) .*?/\1 \2/")
BASE=0x$(echo $PARAMS | cut -d' ' -f 1)
SIZE=$(echo $PARAMS | cut -d' ' -f 2)

LOGBUF=$(mktemp)
SCRIPT_DIR=$(readlink -f $(dirname $0))
openocd \
	-s $SCRIPT_DIR \
	-f digilent-hs1.cfg \
	-c "transport select jtag" \
	-f zybo.cfg \
	-c "adapter_khz 2000" \
	-c "init" \
	-c "halt" \
	-c "dump_image $LOGBUF $BASE $SIZE" \
	-c "resume" \
	-c "shutdown"

echo "------ $LOGBUF"
hd $LOGBUF

