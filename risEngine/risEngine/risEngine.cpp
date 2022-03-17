#include "pch.h"

#include <iostream>

#include "risEngine.h"
#include "../risJobSystem/risJobSystem.h"

namespace risEngine
{
	risEngine::risEngine(risArguments& arguments)
	{
		singleton_janitor_ = risSingletonJanitor();

		auto job_system_parameters = risJobSystemParameters();
		job_system_parameters.threads = arguments.job_threads;

		singleton_janitor_.create<risJobSystem>(reinterpret_cast<uintptr_t>(&job_system_parameters));
	}

	void risEngine::run()
	{
		auto counter = 0;
		while (true)
		{
			std::cout << counter << std::endl;
			if (++counter > 10)
				break;
		}
	}

	risEngine::~risEngine()
	{
		std::cout << "bruh" << std::endl;
	}
}
