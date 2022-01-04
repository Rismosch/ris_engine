#include "../modules/risLog/LogModule.h"

using namespace risLog;

int main()
{
	// startup
	const auto log = new LogModule(LogLevel::Debug);

	// ??
	log->trace("one");
	log->debug("two");
	log->warning("three");
	log->error("four");

	// shutdown
	delete log;
}
