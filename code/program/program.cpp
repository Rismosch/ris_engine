#include <iostream>

#include "flags.h"
#include "../modules/risUtility/risLog.h"
#include "../modules/risUtility/risFlag.h"

using namespace risUtility;

using namespace ris;

int main()
{
	// startup
	const auto log = new risLog(LogLevel::Warning);
	const auto flags = new risFlag();

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

	// shutdown
	delete flags;
	delete log;
}
