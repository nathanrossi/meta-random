#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <arpa/inet.h>

// listen on a tcp socket
// wait to accept
// once accepted fork and manage

int child(int fd)
{
	// wait for data on the connection
	char buf[1024];
	char** execargs = NULL;

	while (1)
	{
		int r = read(fd, buf, 1024);

		if (r == 0)
		{
			printf("child: got finished\n");
			return 0;
		}
		else if (r < 0)
		{
			printf("child: got error\n");
			return -1;
		}

		printf("child: got %d bytes\n", r);

		uint32_t len = ntohl(*((uint32_t*)buf));
		uint8_t type = buf[4];

	}
	return 0;
}

int server()
{
	int port = 2222;
	int fd;

	fd = socket(AF_INET, SOCK_STREAM, 0);
	if (fd < 0)
	{
		exit(-1);
	}

	struct sockaddr_in addr;
	bzero(&addr, sizeof(addr));
	addr.sin_family = AF_INET;
	addr.sin_addr.s_addr = htonl(INADDR_ANY);
	addr.sin_port = htons(port);

	bind(fd, (struct sockaddr*)&addr, sizeof(addr));
	listen(fd, 5);
	printf("server: listening...\n");

	while (1)
	{
		int cfd = accept(fd, NULL, NULL);

		if (cfd < 0)
		{
			printf("server: failed on accept\n");
			return -1;
		}

		// fork to handle connection
		printf("server: forking\n");
		int cpid = fork();
		if (cpid == 0)
		{
			printf("child: exists\n");
			return child(cfd);
		}
		printf("server: made child %d\n", cpid);
		// close the connection on the parent
		close(cfd);
	}

	return 0;
}

int client()
{
	int port = 2222;
	int fd;

	fd = socket(AF_INET, SOCK_STREAM, 0);
	if (fd < 0)
	{
		exit(-1);
	}

	struct sockaddr_in addr;
	bzero(&addr, sizeof(addr));
	addr.sin_family = AF_INET;
	addr.sin_addr.s_addr = inet_addr("127.0.0.1");
	addr.sin_port = htons(port);

	if (connect(fd, (struct sockaddr*)&addr, sizeof(addr)) != 0)
	{
		printf("client: failed to connect\n");
	}

	uint32_t len = 4 + 4 + 1;
	uint8_t buf[1024];
	*(uint32_t*)(buf) = htonl(len);
	buf[4] = (uint8_t)0xff; // is arg

	char* arg = "bash";
	memcpy(buf + 5, arg, strlen(arg));

	write(fd, buf, len);

	close(fd);
	return 0;
}

int main(int argc, char* argv[])
{
#ifdef SERVER
	return server();
#else
	return client();
#endif
}
