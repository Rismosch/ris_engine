#include <iostream>
#include "../risLog/LogModule.h"

using namespace risLog;

int main()
{
	std::cout << "start" << std::endl;

	const auto log = new LogModule(LogLevel::None);

	std::cout << "trace" << std::endl;
	log->trace("one");

	std::cout << "debug" << std::endl;
	log->debug("two");

	std::cout << "warning" << std::endl;
	log->warning("three");

	std::cout << "error" << std::endl;
	log->error("four");

	
	
	std::cout << "reset" << std::endl;
	delete log;
	
	std::cout << "end" << std::endl;
}
