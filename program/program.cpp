#include <iostream>
#include "../risLog/LogModule.h"

using namespace risLog;

int main()
{
	std::cout << "start" << std::endl;
	
	auto log_module = new LogModule();
	
	std::cout << "none" << std::endl;
	std::cout << "trace" << std::endl;
	std::cout << "debug" << std::endl;
	std::cout << "warning" << std::endl;
	std::cout << "error" << std::endl;
	
	
	std::cout << "reset" << std::endl;
	// log_module.reset();
	
	std::cout << "end" << std::endl;
}
