#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/random.h>

int main(int argc, char* argv[])
{
	int bufsize = (sizeof(int) * 2) + 512;
	char* buf = (char*)malloc(bufsize);

	if (fork() != 0)
	{
		printf("daemoning bad entropy\n");
		return 0;
	}

	// populate struct
	((int*)buf)[0] = 512 * 4;
	((int*)buf)[1] = 512;

	printf("starting badentropy...\n");
	int success = 0;
	int fd = open("/dev/random", O_RDWR);
	while (1)
	{
		if (ioctl(fd, RNDADDENTROPY, buf) < 0)
		{
			printf("ioctl failed\n");
			return -1;
		}

		// wait
		struct timespec req;
		req.tv_sec = 0;
		req.tv_nsec = 100 * 1000000;

		nanosleep(&req, NULL);

		if (success == 0)
		{
			printf("badentropy has begun\n");
			success = 1;
		}
	}
	return 0;
}
