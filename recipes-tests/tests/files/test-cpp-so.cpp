#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include <iostream>
#include <vector>

int main(int argc, char* argv[])
{
	std::vector<int> test_vec;

	test_vec.push_back(0);
	test_vec.push_back(32767);
	test_vec.push_back(-32768);

	for (std::vector<int>::iterator it = test_vec.begin(); it != test_vec.end(); ++it)
	{
		std::cout << "testing iostream and c++ vector, value " << *it << std::endl;
	}

	std::cout << "test iostream so linking" << std::endl;

	return 0;
}
