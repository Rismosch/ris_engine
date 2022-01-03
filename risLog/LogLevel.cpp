#include "pch.h"
#include <string>

namespace risLog
{
	enum class LogLevel
	{
		Trace,
		Debug,
		Warning,
		Error
	};

	inline std::string LogLevelToString(LogLevel logLevel)
	{
		switch (logLevel)
		{
		case LogLevel::Trace:
			return "Trace";

		case LogLevel::Debug:
			return "Debug";

		case LogLevel::Warning:
			return "Warning";

		case LogLevel::Error:
			return "Error";
			break;

		default:
			return "UNDEFINED";
		}
	}
}