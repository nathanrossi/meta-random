
do_deploy_append() {
	install -d ${DEPLOY_DIR_IMAGE}
	${TARGET_PREFIX}readelf -a ${D}/boot/vmlinux-* | grep __log_buf | sed -r "s/.*?: ([a-f0-9]*) (0x[0-9]*|[0-9]*) .*?/\1 \2/" > ${DEPLOY_DIR_IMAGE}/logbuf.map
}

