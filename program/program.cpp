#include <iostream>
#include "../risLog/LogModule.h"

risLog::LogModule LogModule;

int main()
{
	risLog::Trace("was");
	risLog::Debug("geht");
	risLog::Warning("ab");
	risLog::Error("?!");
}
