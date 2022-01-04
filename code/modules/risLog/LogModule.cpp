#include "pch.h"
#include "LogModule.h"
#include <iostream>

namespace risLog
{
	struct LogModule::Impl
	{
		LogLevel level;
	};
	
	LogModule::LogModule(LogLevel level) : pImpl(new Impl())
	{
		std::cout << "log module constructor" << std::endl;
		set_log_level(level);
	}

	LogModule::~LogModule()
	{
		std::cout << "log module destructor" << std::endl;
		delete pImpl;
	}

	void LogModule::set_log_level(LogLevel level) const
	{
		pImpl->level = level;
	}

	LogLevel LogModule::get_log_level() const
	{
		return pImpl->level;
	}

	std::string LogModule::level_to_string(LogLevel level)
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
		default:
			return "undefined";
		}
	}

	void LogModule::log(const std::string& message, LogLevel level) const
	{
		if (pImpl->level < level)
			return;

		std::cout << "[" << level_to_string(level) << "," << "hhh:mm:ss" << "] " << message << std::endl;
	}

	void LogModule::error(const std::string& message) const
	{
		log(message, LogLevel::Error);
	}

	void LogModule::warning(const std::string& message) const
	{
		log(message, LogLevel::Warning);
	}

	void LogModule::debug(const std::string& message) const
	{
		log(message, LogLevel::Debug);
	}

	void LogModule::trace(const std::string& message) const
	{
		log(message, LogLevel::Trace);
	}
}
