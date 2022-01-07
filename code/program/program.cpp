#include <iostream>

#include "flags.h"
#include "../modules/risData/crc32.h"
#include "../modules/risData/stringid.h"
#include "../modules/risUtility/risLog.h"
#include "../modules/risUtility/risFlag.h"
#include "../modules/risUtility/StackAllocator.h"

using namespace ris;

using namespace risUtility;

int main()
{
	// startup
	const auto log = new risLog(LogLevel::Warning);
	const auto flags = new risFlag();
	const auto stackAllocator = new StackAllocator(sizeof(U32) * 2);

	// test logger
	log->trace("one");
	log->debug("two");
	log->warning("three");
	log->error("four");

	// test flag
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

	// test stack allocator
	U32* number0 = nullptr;
	U32* number1 = nullptr;
	U32* number2 = nullptr;
	U32* number3 = nullptr;
	StackAllocator::Marker marker = 0;

	number0 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	*number0 = 42;

	marker = stackAllocator->get_marker();

	number1 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << " " << *number1 << " 0 0" << std::endl;
	*number1 = 13;
	std::cout << *number0 << " " << *number1 << " 0 0" << std::endl;

	stackAllocator->free_to_marker(marker);

	number2 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << " " << *number1 << " " << *number2 << " 0" << std::endl;
	*number2 = 0;
	std::cout << *number0 << " " << *number1 << " " << *number2 << " 0" << std::endl;

	stackAllocator->clear();

	number3 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << " " << *number1 << " " << *number2 << " " << *number3 << std::endl;
	*number3 = 7;
	std::cout << *number0 << " " << *number1 << " " << *number2 << " " << *number3 << std::endl;

	// test strings
	auto stringid0 = internString("test1");
	auto stringid1 = internString("wazzup?");
	auto stringid2 = internString("bruh");

	std::cout << stringid0 << " " << stringid1 << " " << stringid2 << std::endl;

	// shutdown
	delete stackAllocator;
	delete flags;
	delete log;
}
