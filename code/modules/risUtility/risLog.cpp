#include "pch.h"
#include "risLog.h"
#include <iostream>

namespace risUtility
{
	struct risLog::Impl
	{
		LogLevel level;
	};
	
	risLog::risLog(LogLevel level) : pImpl(new Impl())
	{
		set_log_level(level);
	}

	risLog::~risLog()
	{
		delete pImpl;
	}

	void risLog::set_log_level(LogLevel level) const
	{
		pImpl->level = level;
	}

	LogLevel risLog::get_log_level() const
	{
		return pImpl->level;
	}

	std::string risLog::level_to_string(LogLevel level)
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

	void risLog::log(const std::string& message, LogLevel level) const
	{
		if (pImpl->level < level)
			return;

		std::cout << "[" << level_to_string(level) << "," << "hhh:mm:ss" << "] " << message << std::endl;
	}

	void risLog::error(const std::string& message) const
	{
		log(message, LogLevel::Error);
	}

	void risLog::warning(const std::string& message) const
	{
		log(message, LogLevel::Warning);
	}

	void risLog::debug(const std::string& message) const
	{
		log(message, LogLevel::Debug);
	}

	void risLog::trace(const std::string& message) const
	{
		log(message, LogLevel::Trace);
	}
}
