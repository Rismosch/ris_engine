#include "pch.h"
#include "risLog.h"

// #include <cstdarg>
// #include <cstdio>

namespace risUtility
{
	const char* log_level_to_string(LogLevel level)
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

	risLog::risLog(LogLevel logLevel) : log_level(logLevel) { }
	risLog::~risLog(){}

	void risLog::log(LogLevel level, const char* file, const int line, const char* message, ...)
	{
		if (log_level < level)
			return;

		auto logLevel = log_level_to_string(level);




		// char buffer[50];
		// int n, a = 5, b = 3;
		//
		// va_list args;
		// va_start(args, message);
		// vprintf(message, args);
		// n = sprintf_s(buffer, "%d plus %d is %d", args);
		// va_end(args);
		//
		// printf("[%s] is a string %d chars long\n", buffer, n);

		// std::cout << "[" << level_to_string(level) << "," << "hhh:mm:ss" << "] " << message << std::endl;
	}

	// inline void risLog::error(const char* message, ...)
	// {
	// 	va_list args;
	// 	va_start(args, message);
	// 	log(LogLevel::Error, __FILE__, __LINE__, message, args);
	// 	va_end(args);
	// }

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
