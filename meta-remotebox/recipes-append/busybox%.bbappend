

do_prepare_config_append () {
	sed -i 's/^.*CONFIG_TFTPD.*$/CONFIG_TFTPD=y/g' ${S}/.config
	sed -i 's/^.*CONFIG_UDPSVD.*$/CONFIG_UDPSVD=y/g' ${S}/.config
}

