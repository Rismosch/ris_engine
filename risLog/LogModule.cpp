#include "pch.h"
#include "framework.h"
#include "LogModule.h"
#include <iostream>

namespace risLog
{
	struct LogModule::Impl
	{
		int log_level;
	};

	LogModule::LogModule() : pImpl(new Impl())
	{
		std::cout << "log module constructor" << std::endl;
		set_log_level(4);
	}

	LogModule::~LogModule()
	{
		std::cout << "log module destructor" << std::endl;
		delete pImpl;
	}

	void LogModule::set_log_level(int log_level) const
	{
		pImpl->log_level = log_level;
	}

	int LogModule::get_log_level() const
	{
		return pImpl->log_level;
	}

	void LogModule::log(const std::string& message, int log_level) const
	{
		if (pImpl->log_level < log_level)
			return;

		std::cout << "[" << log_level << ","<< "hhh:mm:ss" << "] " << message << std::endl;
	}

	inline void LogModule::trace(const std::string& message) const
	{
		log(message, 4);
	}

	inline void LogModule::debug(const std::string& message) const
	{
		log(message, 3);
	}

	inline void LogModule::warning(const std::string& message) const
	{
		log(message, 2);
	}

	inline void LogModule::error(const std::string& message) const
	{
		log(message, 1);
	}
}
