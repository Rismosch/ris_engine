#include "pch.h"
#include "risLog.h"

#include <cstdarg>
#include <cstdio>

namespace risUtility
{
	risLog::risLog(LogLevel logLevel) : log_level(logLevel) { }

	inline const char* risLog::level_to_string(LogLevel level)
	{
		switch (level)
		{
		case LogLevel::None:
			return "None";
		case LogLevel::Error:
			return "Error";
		case LogLevel::Warning:
			return "Warning";
		case LogLevel::Debug:
			return "Debug";
		case LogLevel::Trace:
			return "Trace";
		default:  // NOLINT(clang-diagnostic-covered-switch-default)
			return "undefined";
		}
	}

	inline void risLog::log(LogLevel level, const char* file, const int line, const char* message, ...)
	{
		if (log_level < level)
			return;

		auto bruh = level_to_string(level);
		va_list args;
		va_start(args, message);
		vprintf(message, args);
		va_end(args);

		// std::cout << "[" << level_to_string(level) << "," << "hhh:mm:ss" << "] " << message << std::endl;
	}

	inline void risLog::error(const char* message, ...)
	{
		va_list args;
		va_start(args, message);
		log(LogLevel::Error, __FILE__, __LINE__, message, args);
		va_end(args);
	}

	// void risLog::warning(const char* message) const
	// {
	// 	log(message, LogLevel::Warning);
	// }
	//
	// void risLog::debug(const char* message) const
	// {
	// 	log(message, LogLevel::Debug);
	// }
	//
	// void risLog::trace(const char* message) const
	// {
	// 	log(message, LogLevel::Trace);
	// }
}
