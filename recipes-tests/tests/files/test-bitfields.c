#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define assert_type(x, y, t, tf) \
	do { \
		if (x != y) { \
			printf("assert failed: %s = %" tf ", expecting %" tf "\n", #x, (t)x, (t)y); \
			abort(); \
		} \
		printf("assert passed: %s == %" tf " == %" tf "\n", #x, (t)x, (t)y); \
	\
	} while (0)
#define assert_s(x, y) assert_type(x, y, long long, "lld")
#define assert_u(x, y) assert_type(x, y, unsigned long long, "llu")

typedef struct {
	int fd;
	unsigned int created : 1;
	unsigned int readable : 1;
	unsigned int writable : 1;
	unsigned int appending : 1;
	signed int seekable : 2; /* -1 means unknown */
	unsigned int closefd : 1;
	char finalizing;
	unsigned int blksize;
} test;

// Setup some functions that modify/validate the bitfields these functions are
//
// Setup as noinline to avoid any optimizations made by register use and
// expression optimization. This forces the code to pull the bitfield from the
// memory location, or store it there.

void __attribute__ ((noinline)) setup(test* v)
{
	v->fd = -1;
	v->created = 0;
	v->readable = 0;
	v->writable = 0;
	v->appending = 0;
	v->seekable = -1;
	v->blksize = 0;
	v->closefd = 1;
}

void __attribute__ ((noinline)) modify(test* v, int val)
{
	v->seekable = val;
}

void __attribute__ ((noinline)) verify_single(test* v, int val)
{
	int f = v->seekable;
	if (f != val)
		abort();
}

void __attribute__ ((noinline)) verify(test* v, int val)
{
	assert_u(v->created, 0);
	assert_u(v->readable, 0);
	assert_u(v->writable, 0);
	assert_u(v->appending, 0);
	verify_single(v, val);
	assert_s(v->seekable, val);
	assert_u(v->blksize, 0);
	assert_u(v->closefd, 1);
}

int main(void)
{
	test* v = malloc(sizeof(test));
	setup(v);

	verify(v, -1);

	/* change to 0 */
	modify(v, 0);
	verify(v, 0);

	/* change to 1 */
	modify(v, 1);
	verify(v, 1);

	return 0;
}
