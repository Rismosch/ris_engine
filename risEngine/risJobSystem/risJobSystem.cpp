#include "pch.h"

#include "risJobSystem.h"

#include <iostream>

namespace risEngine
{
	risJobSystem* risJobSystem::p_instance_;

	risJobSystem* risJobSystem::instance()
	{
		return p_instance_;
	}


	void risJobSystem::create()
	{
		std::cout << "create job system" << std::endl;

		std::cout << "threads to spawn: " << "??" << std::endl;
	}

	void risJobSystem::destroy()
	{
		std::cout << "destroy job system" << std::endl;
	}
}
