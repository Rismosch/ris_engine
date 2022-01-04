#pragma once
#include <string>

namespace risLog
{
	class LogModule
	{
	public:
		LogModule();
		~LogModule();

		void set_log_level(int log_level) const;
		int get_log_level() const;

		void log(const std::string& message, int log_level) const;
		inline void trace(const std::string& message) const;
		inline void debug(const std::string& message) const;
		inline void warning(const std::string& message) const;
		inline void error(const std::string& message) const;

	private:
		struct Impl;
		Impl* pImpl{};
	};
}