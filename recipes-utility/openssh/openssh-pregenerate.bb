DESCRIPTION = "Pregenerate the targets SSH host keys"
LICENSE = "MIT"
#DEPENDS = "openssh-native"

do_configure() {
	:
}

KEYS = "rsa dsa ecdsa ed25519"

do_compile[dirs] += "${B}"
do_compile() {
	export PATH=/usr/bin:$PATH
	for i in ${KEYS}; do
		if [ -e ${B}/ssh_host_${i}_key ]; then
			rm ${B}/ssh_host_${i}_key
		fi
		ssh-keygen -q -f ${B}/ssh_host_${i}_key -N '' -t $i
	done
}

do_install() {
	for i in ${KEYS}; do
		install -Dm 0600 ${B}/ssh_host_${i}_key ${D}/${sysconfdir}/ssh/ssh_host_${i}_key
	done
}
