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

int testcase_subdi3_reg64_regs(void)
{
	uint64_t a = 100;
	uint64_t b64 = 6;
	uint32_t b32 = 6;

	assert_u((a - b64), 94UL);
	assert_u((a - b32), 94UL);

	return 0;
}

int testcase_subdi3_reg64_consts(void)
{
	uint64_t a = 0x20000000UL;

	assert_u((a - 0x10000000UL), 0x10000000UL);
	assert_u((a - 0x10000), 0x1fff0000UL);
	assert_u((a - ((uint16_t)0x10)), 0x1ffffff0UL);

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
	run_testcase(testcase_subdi3_reg64_regs);
	run_testcase(testcase_subdi3_reg64_consts);

	return 0;
}
