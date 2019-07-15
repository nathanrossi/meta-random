#ifndef TEST_H
#define TEST_H

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>

#define assert_u(x,y) if (__assert_u(x, y, #x) != 0) return -1;
#define assert_s(x,y) if (__assert_s(x, y, #x) != 0) return -1;

int __assert_u(uint64_t x, uint64_t y, const char* statement)
{
	if (x != y) {
		printf("assert failed: %s = %" PRIu64 ", expecting %" PRIu64 "\n", statement, x, y);
		return -1;
	}
	printf("assert passed: %s == %" PRIu64 " == %" PRIu64 "\n", statement, x, y);
	return 0;
}

int __assert_s(int64_t x, int64_t y, const char* statement)
{
	if (x != y) {
		printf("assert failed: %s = %" PRIu64 ", expecting %" PRIu64 "\n", statement, x, y);
		return -1;
	}
	printf("assert passed: %s == %" PRIu64 " == %" PRIu64 "\n", statement, x, y);
	return 0;
}

#define run_testcase(x) if (__run_testcase(#x, x) != 0) return -1;
int __run_testcase(char* name, int (*tcfunc)(void))
{
	printf("%s: running\n", name);
	if (tcfunc() == 0)
	{
		printf("%s: PASSED\n", name);
		return 0;
	}
	printf("%s: FAILED\n", name);
	return -1;
}

#endif
