#include <iostream>

#include "../modules/risUtility/LogModule.h"
#include "../modules/risUtility/FlagModule.h"

using namespace risLog;
using namespace risFlag;

using ::flag;

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
	flags->toggle(flag::Test0);
	flags->toggle(flag::Test2);

	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->set(flag::Test1, true);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->set(flag::Test1, false);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->toggle(flag::Test1);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->toggle(flag::Test1);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->toggle(flag::Test2);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;
	flags->toggle(flag::Test2);
	std::cout << flags->to_string() << " Flag1: " << flags->get(flag::Test1) << std::endl;

	// shutdown
	delete flags;
	delete log;
}
