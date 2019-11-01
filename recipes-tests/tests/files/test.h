#ifndef TEST_H
#define TEST_H

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

//#define DEBUG

#define assert_u(x,y) if (__uasserteq(x, y, #x) != 0) return -1;
#define assert_s(x,y) if (__sasserteq(x, y, #x) != 0) return -1;

#define uasserteq(x,y) if (__uasserteq(x, y, #x) != 0) return -1;
#define uassertne(x,y) if (__uassertne(x, y, #x) != 0) return -1;
#define sasserteq(x,y) if (__sasserteq(x, y, #x) != 0) return -1;
#define sassertne(x,y) if (__sassertne(x, y, #x) != 0) return -1;

#ifdef DEBUG
#define __print_debug(x) x
#else
#define __print_debug(x)
#endif

#define __assert_template(func, type, op, words) \
	int func(type x, type y, const char* statement) \
{ \
	if (!(x op y)) { \
		printf("assert failed: %s = %" PRIu64 ", " words " %" PRIu64 "\n", statement, x, y); \
		return -1; \
	} \
	__print_debug(printf("assert passed: %s == %" PRIu64 " " #op " %" PRIu64 "\n", statement, x, y);) \
	return 0; \
}

__assert_template(__uasserteq, uint64_t, ==, "expecting")
__assert_template(__uassertne, uint64_t, !=, "not expecting")
__assert_template(__sasserteq, int64_t, ==, "expecting")
__assert_template(__sassertne, int64_t, !=, "not expecting")

void assert_fatal(void)
{
	printf("assert failed: fatal, aborting\n");
	abort();
}

#define run_testcase(x) if (__run_testcase(#x, x) != 0) return -1;
int __run_testcase(char* name, int (*tcfunc)(void))
{
	printf("%s: running\n", name);
	if (tcfunc() == 0)
	{
		printf("%s: PASSED\a\n", name);
		return 0;
	}
	printf("%s: FAILED\a\n", name);
	return -1;
}

#endif
