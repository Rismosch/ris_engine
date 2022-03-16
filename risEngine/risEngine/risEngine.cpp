#include "pch.h"

#include <iostream>

#include "risEngine.h"
#include "../risJobSystem/risJobSystem.h"

namespace risEngine
{
	risEngine::risEngine(risArguments& arguments)
	{
		singleton_janitor_ = risSingletonJanitor();
		singleton_janitor_.create<risJobSystem>();
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
