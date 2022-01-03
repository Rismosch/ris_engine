#include "pch.h"
#include "framework.h"
#include "LogModule.h"
#include <iostream>

namespace risLog
{
	LogModule::LogModule() : pImpl(new Impl) {}
	LogModule::~LogModule() { delete pImpl; }

	struct LogModule::Impl
	{

	};

	void LogModule::setUp()
	{

	}

	void LogModule::shutDown()
	{

	}
}