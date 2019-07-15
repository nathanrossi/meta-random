#include "test.h"

int testcase_longlong_words_check(void)
{
#define VALUE 0xF0000000ULL
	unsigned long long a = VALUE;
	uint32_t lw = a & 0xffffffff;
	uint32_t hw = (a >> 32) & 0xffffffff;

	assert_u(lw, 0xf0000000UL);
	assert_u(hw, 0x00000000UL);

	return 0;
}

int testcase_moddi3(void)
{
	int64_t divisor = 1000000002;
	int32_t remainder;
	int64_t quotient;

#define FIXED_VALUE_SOMETHING 1000000000
	remainder = divisor % FIXED_VALUE_SOMETHING;
	quotient = divisor / FIXED_VALUE_SOMETHING;

	assert_s(remainder, 2);
	assert_s(quotient, 1);

	return 0;
}

int main(int argc, char* argv[])
{
	run_testcase(testcase_longlong_words_check);
	run_testcase(testcase_moddi3);
	return 0;
}
