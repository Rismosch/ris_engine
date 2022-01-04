#pragma once
#include <string>

namespace risLog
{
	enum class LogLevel
	{
		None,
		Error,
		Warning,
		Debug,
		Trace
	};

	class LogModule
	{
	public:
		LogModule(LogLevel level);
		~LogModule();

		inline static std::string level_to_string(LogLevel level);

		void set_log_level(LogLevel level) const;
		LogLevel get_log_level() const;
		
		void log(const std::string& message, LogLevel level) const;
		void trace(const std::string& message) const;
		void debug(const std::string& message) const;
		void warning(const std::string& message) const;
		void error(const std::string& message) const;

	private:
		struct Impl;
		Impl* pImpl{};
	};
}