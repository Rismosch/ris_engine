#include "pch.h"
#include <risEngine/data/string.hpp>
#include <risEngine/engine/engine.hpp>
#include <risEngine/time/stopwatch.hpp>
#include <risEngine/job_system/job_system.hpp>

#include <iostream>

namespace risEngine
{
	risEngine::risEngine(const risArguments& arguments)
	{
		singleton_janitor_ = risSingletonJanitor();
		
		auto job_system_parameters = risJobSystemParameters();
		job_system_parameters.threads = arguments.job_threads;
		
		singleton_janitor_.create<risJobSystem>(reinterpret_cast<uintptr_t>(&job_system_parameters));
	}

	void risEngine::run()
	{
		const auto stopwatch = risStopwatch();

		auto counter = 0;
		while (true)
		{
			std::cout << counter << std::endl;
			if (++counter > 40000)
				break;
		}

		const auto elapsed = stopwatch.elapsed();

		std::cout << "elapsed: " << elapsed << std::endl;
	}

	risEngine::~risEngine()
	{
		std::cout << "bruh " << sid("bruh") << std::endl;
	}
}
