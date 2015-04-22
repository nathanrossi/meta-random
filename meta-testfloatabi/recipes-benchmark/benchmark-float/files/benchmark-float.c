#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <time.h>

float test_float_lib_cc_func(float a, float b);

void test_float(void)
{
	int c;
	for (c = 0; c < 1000; c++)
	{
		int i;
		float a = 1234.1234;
		float b = 4321.4321;
		for (i = 0; i < 1000; i++)
		{
			a = (a * 2) / b;
		}
	}
}

float __attribute__ ((noinline)) test_float_local_cc_func(float a, float b)
{
	return (a * 2) / b;
}

void test_float_local_cc(void)
{
	int c;
	for (c = 0; c < 1000; c++)
	{
		int i;
		float a = 1234.1234;
		float b = 4321.4321;
		for (i = 0; i < 1000; i++)
		{
			a = test_float_local_cc_func(a, b);
		}
	}
}

void test_float_lib_cc(void)
{
	int c;
	for (c = 0; c < 1000; c++)
	{
		int i;
		float a = 1234.1234;
		float b = 4321.4321;
		for (i = 0; i < 1000; i++)
		{
			a = test_float_lib_cc_func(a, b);
		}
	}
}

uint64_t diff_us(struct timespec a, struct timespec b)
{
	uint64_t temp_a, temp_b;
	temp_a = a.tv_sec * 1000000;
	temp_a += a.tv_nsec / 1000;
	temp_b = b.tv_sec * 1000000;
	temp_b += b.tv_nsec / 1000;

	if (temp_a > temp_b)
		return temp_a - temp_b;
	return temp_b - temp_a;
}

void execute(void (*func)(void), int runs, char* testname)
{
	uint64_t avg = 0;
	int run;

	for (run = 0; run < runs; run++)
	{
		struct timespec start, end;
		clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &start);
		if (func != NULL)
			func();
		clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end);

		avg = (avg + diff_us(start, end)) / 2;
	}

	printf("%s - %llu us (avg over %d runs)\n", testname, avg, runs);
}

int main(void)
{
	printf("Benchmark Simple Floating Point Operations\n");

	execute(test_float, 1000, "test_float");
	execute(test_float_local_cc, 1000, "test_float_local_cc");
	execute(test_float_lib_cc, 1000, "test_float_lib_cc");

	printf("done\n");
	return 0;
}

