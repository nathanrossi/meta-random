#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>

void fill_pattern(int* mem, int size)
{
	/* fill the memory location with a pattern */
	int a;
	for (a = 0; a < size; a++)
	{
		mem[a] = a;
	}
}

int test_pattern(int* mem, int size)
{
	int a;
	for (a = 0; a < size; a++)
	{
		if (mem[a] != a)
			return -1;
	}
	return 0;
}

int loop(int count)
{
	int blocksize = 4096 / sizeof(int); // 1 page
	int newcount = count;
	int freed = 0;

	if (count == 0)
		goto return_inc;

	// allocate storage
	int** pages = (int**)malloc(sizeof(int*) * count);
	memset(pages, 0x0, sizeof(int*) * count);
	printf("% 10d: allocated", count); fflush(stdout);

	// allocate
	printf(", fill"); fflush(stdout);
	for (int a = 0; a < count; a++)
	{
		pages[a] = (int*)malloc(sizeof(int) * blocksize);
		if (pages[a] == NULL)
		{
			// check for failed malloc
			printf("\n% 10d: reached count %d, before malloc failed/oom'd\n", count, count); fflush(stdout);
			newcount = a;
			goto cleanup;
		}
		memset(pages[a], 0x0, sizeof(int) * blocksize);
		fill_pattern(pages[a], blocksize);
	}

	// test
	printf(", test"); fflush(stdout);
	for (int a = 0; a < count; a++)
	{
		if (test_pattern(pages[a], blocksize))
		{
			printf("\n% 10d: failed test pattern %d\n", count, a); fflush(stdout);
			abort();
		}
	}

cleanup:
	printf(", free"); fflush(stdout);
	for (int a = 0; a < count; a++)
	{
		if (pages[a] != NULL)
		{
			free(pages[a]);
			freed += 1;
		}
	}
	printf(", freed %d", freed); fflush(stdout);

	free(pages);
	printf(", free pages\n"); fflush(stdout);

return_inc:
	return newcount + 1024;
}

void prevent_overcommit(void)
{
	char* data = "2";
	int f = open("/proc/sys/vm/overcommit_memory", O_WRONLY);
	if (f < 0)
		printf("failed to open for overcommit\n");
	if (write(f, data, 1) != 1)
		printf("failed to write to overcommit\n");
}

int main(void)
{
	prevent_overcommit();
	// test memory via ballooning small and large malloc operations forever, or
	// until an error occurs in which then abort.
	int count = 0;
	while (1)
	{
		count = loop(count);
	}
	return 0;
}

