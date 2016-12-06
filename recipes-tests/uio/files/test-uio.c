#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <assert.h>
#include <sys/mman.h>
#include <fcntl.h>

int main(int argc, char *argv[])
{
	int fd;
	char* uio_dev = "/dev/uio0";
	int uio_size = 0x1000; /* hardcode, can read from sysfs */
	volatile long* mmap_ptr;

	/* open */
	fd = open(uio_dev, O_RDWR);
	if (fd < 1) {
		perror(argv[0]);
		return -1;
	}

	/* mmap */
	mmap_ptr = (volatile long*)mmap(NULL, uio_size, PROT_READ|PROT_WRITE, MAP_SHARED, fd, 0);
	if (!mmap_ptr) {
		perror(argv[0]);
		return -1;
	}

	long addr = 0;
	while (1) {
		/* test read/write @ 0x0 */
		printf("read(0x%08x): %08x\n", addr, mmap_ptr[addr]);

		printf("writ(0x%08x): 0xdeadbeef\n", addr);
		mmap_ptr[addr] = 0xdeadbeef;

		printf("read(0x%08x): %08x\n", addr, mmap_ptr[addr]);

		sleep(1);

		addr++;
		addr = addr % (uio_size / sizeof(long));
	}

	/* unmmap */
	munmap((void*)mmap_ptr, uio_size);
	return 0;
}
