#include <stdio.h>
#include <stdint.h>

#define __return_fail(x) do { if ((x) != 0) return -1; } while (0)

#define assert(x)       assert_true(x)
#define assert_fail()   __return_fail(__assert_fail(__func__))
#define assert_true(x)  __return_fail(__assert_true((x), __func__, #x))
#define assert_false(x) __return_fail(__assert_false((x), __func__, #x))
#define assert_ueq(x,y) __return_fail(__assert_eq_ulong(x, y, __func__, #x, #y))
#define assert_une(x,y) __return_fail(__assert_ne_ulong(x, y, __func__, #x, #y))

int __assert_fail(const char* caller)
{
	printf("%s: assert failed\n", caller);
	return -1;
}

int __assert_true(const int val, const char* caller, const char* lhs_s)
{
	if (val)
	{
		printf("%s: assert passed: %s is true\n", caller, lhs_s);
		return 0;
	}
	printf("%s: assert failed: %s is false (expected true)\n", caller, lhs_s);
	return -1;
}

int __assert_false(const int val, const char* caller, const char* lhs_s)
{
	if (!val)
	{
		printf("%s: assert passed: %s is false\n", caller, lhs_s);
		return 0;
	}
	printf("%s: assert failed: %s is true (expected false)\n", caller, lhs_s);
	return -1;
}

int __assert_eq_ulong(const unsigned long lhs_v, const unsigned long rhs_v, const char* caller, const char* lhs_s, const char* rhs_s)
{
	if (lhs_v == rhs_v)
	{
		printf("%s: assert passed: %s == %s (expected %lu, is %lu)\n", caller, lhs_s, rhs_s, rhs_v, lhs_v);
		return 0;
	}
	printf("%s: assert failed: %s == %s (expected %lu, but is %lu)\n", caller, lhs_s, rhs_s, rhs_v, lhs_v);
	return -1;
}

int __assert_ne_ulong(const unsigned long lhs_v, const unsigned long rhs_v, const char* caller, const char* lhs_s, const char* rhs_s)
{
	if (lhs_v != rhs_v)
	{
		printf("%s: assert passed: %s != %s (expected not %lu, is %lu)\n", caller, lhs_s, rhs_s, rhs_v, lhs_v);
		return 0;
	}
	printf("%s: assert failed: %s != %s (expected not %lu, but is %lu)\n", caller, lhs_s, rhs_s, rhs_v, lhs_v);
	return -1;
}

int testcase_bool_compare_and_swap(void)
{
	unsigned int val = 0;

	for (int i = 0; i < 10; i++)
	{
		val = 0;
		assert_ueq(val, 0);
		assert_true(__sync_bool_compare_and_swap(&val, 0, 2));
		assert_ueq(val, 2);
	}

	assert_ueq(val, 2);
	assert_false(__sync_bool_compare_and_swap(&val, 0, 2));
	assert_ueq(val, 2);

	return 0;
}

static int external_val = 0;

int testcase_val_compare_and_swap_static(void)
{
	unsigned int old;

	assert_ueq(external_val, 0);
	old = __sync_val_compare_and_swap(&external_val, 0, 2);
	assert_ueq(external_val, 2);
	assert_ueq(old, 0);

	return 0;
}

int testcase_val_compare_and_swap_static_2(void)
{
	unsigned int old;

	assert_ueq(external_val, 2);
	old = __sync_val_compare_and_swap(&external_val, 2, 4);
	assert_ueq(external_val, 4);
	assert_ueq(old, 2);

	return 0;
}

int testcase_val_compare_and_swap(void)
{
	unsigned int val = 0;
	unsigned int old = 0;

	assert_ueq(val, 0);
	old = __sync_val_compare_and_swap(&val, 0, 2);
	assert_ueq(val, 2);
	assert_ueq(old, 0);

	old = __sync_val_compare_and_swap(&val, 2, 4);
	assert_ueq(val, 4);
	assert_ueq(old, 2);

	return 0;
}

int testcase_val_compare_and_swap_local(void)
{
	unsigned int val = 0;

	assert_ueq(val, 0);
	assert(__sync_val_compare_and_swap(&val, 0, 2) == 0);
	assert_ueq(val, 2);

	assert(__sync_val_compare_and_swap(&val, 2, 4) == 2);
	assert_ueq(val, 4);

	return 0;
}

int testcase_atomic_exchange(void)
{
	unsigned int val = 0;

	assert_ueq(val, 0);
	assert(__atomic_exchange_n(&val, 2, __ATOMIC_ACQUIRE) == 0);
	assert_ueq(val, 2);

	assert_ueq(val, 2);
	assert(__atomic_exchange_n(&val, 4, __ATOMIC_ACQUIRE) == 2);
	assert_ueq(val, 4);

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

int main(int argc, char** argv)
{
	run_testcase(testcase_bool_compare_and_swap);
	run_testcase(testcase_val_compare_and_swap);
	run_testcase(testcase_val_compare_and_swap_local);
	run_testcase(testcase_val_compare_and_swap_static);
	run_testcase(testcase_val_compare_and_swap_static_2);
	run_testcase(testcase_atomic_exchange);
	return 0;
}
