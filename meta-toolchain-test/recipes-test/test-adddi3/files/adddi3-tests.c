#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define assert_u(x,y) \
	do { \
		uint64_t r = x; \
		if (r != y) { \
			printf("assert failed: %s = %llu, expecting %llu\n", #x, r, (uint64_t)y); \
			return -1; \
		} \
		printf("assert passed: %s == %llu == %llu\n", #x, r, (uint64_t)y); \
	\
	} while (0)
#define assert_s(x,y) \
	do { \
		int64_t r = x; \
		if (r != y) { \
			printf("assert failed: %s = %lld, expecting %lld\n", #x, r, (int64_t)y); \
			return -1; \
		} \
		printf("assert passed: %s == %lld == %lld\n", #x, r, (int64_t)y); \
	\
	} while (0)

int testcase_adddi3_reg64_regs(void)
{
	uint64_t a = 4;
	uint64_t b64 = 6;
	uint32_t b32 = 6;

	assert_u((a + b64), 10UL);
	assert_u((a + b32), 10UL);

	return 0;
}

int testcase_adddi3_reg64_consts(void)
{
	uint64_t a = 4;

	assert_u((a + 0x100000000UL), 0x100000004UL);
	assert_u((a + 0x10000), 0x10004UL);
	assert_u((a + ((uint16_t)10)), 14UL);

	return 0;
}

int testcase_adddi3_reg64_const64_gt48b(void)
{
	uint64_t a = 4;

	assert_u((a + 0x1000010000000UL), 0x1000010000004UL);
	assert_u((a + 0x1000000000000UL), 0x1000000000004UL);

	return 0;
}

int testcase_adddi3_reg64_const_ranges(void)
{
	int64_t a = 9000;
	uint64_t b = 9000;

	assert_s((a + (-2147483648L)), -2147474648L);
	assert_s((a + (-65535L)), -56535L);
	assert_s((a + (-32768)), -23768);
	assert_s((a + (-1)), 8999);
	assert_s((a + (0)), 9000);
	assert_s((a + (1)), 9001);
	assert_s((a + (32767)), 41767);
	assert_s((a + (65535L)), 74535);
	assert_s((a + (2147483647L)), 2147492647);

	assert_u((a + (0)), 9000);
	assert_u((a + (1)), 9001);
	assert_u((a + (32767)), 41767);
	assert_u((a + (65535UL)), 74535);
	assert_u((a + (2147483647UL)), 2147492647UL);
	assert_u((a + (0xffffffffUL)), 4294976295ULL);
	assert_u((a + (0xffffffffffffULL)), 281474976719655ULL);
	assert_u((a + (0xffffffffffffffffULL)), 8999);

	return 0;
}

#define run_testcase(x) __run_testcase(#x, x)
void __run_testcase(char* name, int (*tcfunc)(void))
{
	int res = tcfunc();
	if (res == 0)
	{
		printf("%s: PASSED\n", name);
	}
	else
	{
		printf("%s: FAILED\n", name);
	}
}

int main(int argc, char* argv[])
{
	run_testcase(testcase_adddi3_reg64_regs);
	run_testcase(testcase_adddi3_reg64_consts);
	run_testcase(testcase_adddi3_reg64_const64_gt48b);
	run_testcase(testcase_adddi3_reg64_const_ranges);

	return 0;
}
