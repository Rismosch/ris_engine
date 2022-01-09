#include <iostream>

#include "flags.h"
#include "../modules/risData/crc32.h"
#include "../modules/risData/risString.h"
#include "../modules/risUtility/risLog.h"
#include "../modules/risUtility/risFlag.h"
#include "../modules/risUtility/risAllocator.h"
#include "../modules/risUtility/risRandom.h"

using namespace ris;
using namespace risUtility;

risLog* logger;
risFlag* flags;
risAllocator* stackAllocator;
risRandom* rng;

void test_logger();
void test_flag();
void test_allocator();
void test_strings();
void test_rng();

int main()
{
	// startup
	logger = new risLog(LogLevel::Warning);
	flags = new risFlag();
	stackAllocator = new risAllocator(sizeof(U32) * 2);
	rng = new risRandom(42);

	// tests
	test_logger();
	test_flag();
	test_allocator();
	test_strings();
	// test_rng();

	// shutdown
	delete rng;
	delete stackAllocator;
	delete flags;
	delete logger;
}


void test_logger()
{
	std::cout << "\nlogger:" << std::endl;

	logger->trace("one");
	logger->debug("two");
	logger->warning("three");
	logger->error("four");
}

void test_flag()
{
	std::cout << "\nflag:" << std::endl;

	flags->toggle(test0);
	flags->toggle(test2);

	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, true);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, false);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
}

void test_allocator()
{
	std::cout << "\nallocator:" << std::endl;

	U32* number0 = nullptr;
	U32* number1 = nullptr;
	U32* number2 = nullptr;
	U32* number3 = nullptr;
	risAllocator::Marker marker = 0;

	number0 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	*number0 = 42;

	marker = stackAllocator->get_marker();

	number1 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;
	*number1 = 13;
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;

	stackAllocator->free_to_marker(marker);

	number2 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;
	*number2 = 0;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;

	stackAllocator->clear();

	number3 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
	*number3 = 7;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
}

void test_strings()
{
	std::cout << "\nstrings:" << std::endl;

	auto stringid0 = internal_string_to_sid("test1");
	auto stringid1 = internal_string_to_sid("wazzup?");
	auto stringid2 = internal_string_to_sid("bruh");

	std::cout << stringid0 << " " << stringid1 << " " << stringid2 << std::endl;

	auto string0 = sid_to_string(stringid0);
	auto string1 = sid_to_string(stringid1);
	auto string2 = sid_to_string(stringid2);

	if (string0 == nullptr)
		string0 = "null";
	if (string1 == nullptr)
		string1 = "null";
	if (string2 == nullptr)
		string2 = "null";

	std::cout << string0 << " " << string1 << " " << string2 << std::endl;

	std::cout << "shouldn't exist: " << (sid_to_string(static_cast<StringId>(42)) == nullptr) << " (there should be a 1)" << std::endl;
}

void test_rng()
{
	std::cout << "\nrng:" << std::endl;

	for (U16 i = 0; i < 1000; ++i)
	{
		std::cout << rng->bRandom() << " " << rng->fRandom() << " " << rng->iRandom(-24, 13) << std::endl;
	}
}