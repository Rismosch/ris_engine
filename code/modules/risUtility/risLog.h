#pragma once

#include <iostream>
#include "../risData/risData.h"

#define error(message, arguments) log(LogLevel::Error, __FILE__, __LINE__, message, arguments)

namespace risUtility
{
	using namespace risData;

	enum class LogLevel
	{
		None,
		Error,
		Warning,
		Debug,
		Trace
	};

	inline const char* log_level_to_string(LogLevel level);

	class risLog
	{
	public:
		LogLevel log_level;

		risLog(LogLevel level);
		~risLog();


		template <typename T>
		void func(T t)
		{
			std::cout << t << std::endl;
		}

		template<typename T, typename... Args>
		void func(T t, Args... args) // recursive variadic function
		{
			if (recursiveLoops++ == 0)
				std::cout << "start " << std::endl;

			func(t);

			func(args...);


			if(--recursiveLoops == 0)
				std::cout << "end" << std::endl;
		}

		U32 recursiveLoops;

		// inline void error(const char* message, ...);
		// void warning(const char* message) const;
		// void debug(const char* message) const;
		// void trace(const char* message) const;

		void log(LogLevel level, const char* file, const int line, const char* message, ...);
	};
}