#include <iostream>

#include "flags.h"
#include "../modules/risUtility/LogModule.h"
#include "../modules/risUtility/FlagModule.h"

using namespace risUtility;

using namespace ris;

int main()
{
	// startup
	const auto log = new LogModule(LogLevel::Warning);
	const auto flags = new FlagModule();

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
