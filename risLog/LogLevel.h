#pragma once
#include <string>

namespace risLog
{
	class LogLevel
	{
	public:
		LogLevel(int level);

		int getLevel();
		void setLevel(int level);

		std::string toString();

	private:
		struct Impl;
		Impl* pImpl;
	};
}