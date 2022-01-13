#pragma once

// #define error(message, ...) log(LogLevel::Error, __FILE__, __LINE__, message, __VA_ARGS__)

namespace risUtility
{
	enum class LogLevel
	{
		None,
		Error,
		Warning,
		Debug,
		Trace
	};

	class risLog
	{
	public:
		LogLevel log_level;

		risLog(LogLevel level);
		~risLog();

		inline const char* level_to_string(LogLevel level);

		inline void error(const char* message, ...);
		// void warning(const char* message) const;
		// void debug(const char* message) const;
		// void trace(const char* message) const;

		inline void log(LogLevel level, const char* file, const int line, const char* message, ...);
	};
}