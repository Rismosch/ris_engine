#include "pch.h"
#include <string>
#include "LogLevel.h"

namespace risLog
{
	struct LogLevel::Impl
	{
		int level;
	};

	LogLevel::LogLevel(int level): pImpl(new Impl())
	{
		setLevel(level);
	}

	int LogLevel::getLevel()
	{
		return pImpl->level;
	}
	void LogLevel::setLevel(int level)
	{
		pImpl->level = level;
	}

	std::string LogLevel::toString()
	{
		switch (pImpl->level)
		{
		case 0:
			return "Trace";
		case 1:
			return "Debug";
		case 2
		default:
			break;
		}
	}
}