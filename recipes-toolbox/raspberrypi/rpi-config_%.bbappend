do_deploy_append() {
	echo "dwc_otg.lpm_enable=0 console=ttyS1,115200 earlycon" > ${DEPLOYDIR}/bcm2835-bootfiles/cmdline.txt
}
