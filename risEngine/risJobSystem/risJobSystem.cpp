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

	void risJobSystem::create(uintptr_t param)
	{
		auto p_param = reinterpret_cast<risJobSystemParameters*>(param);

		std::cout << "create job system" << std::endl;

		std::cout << "threads to spawn: " << p_param->threads << std::endl;
	}

	void risJobSystem::destroy()
	{
		std::cout << "destroy job system" << std::endl;
	}
}
