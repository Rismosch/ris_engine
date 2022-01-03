#pragma once
#include "../risSupport/risModule.h"
#include "LogLevel.cpp"

using namespace risSupport;

namespace risLog
{
	class LogModule : risModule
	{
	public:
		void setUp() override;
		void shutDown() override;

		void setLogLevel(LogLevel logLevel);
		LogLevel getLogLevel();
	private:
		struct Impl;
		Impl* pImpl;
	};
}