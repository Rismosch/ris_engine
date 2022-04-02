#include "pch.h"
#include <risEngine/job_system/job_system.hpp>

#include <iostream>

namespace risEngine
{
	risJobSystem risJobSystem::instance_;

	risJobSystem& risJobSystem::instance()
	{
		return instance_;
	}

	void risJobSystem::create(uintptr_t param)
	{
		std::cout << "create job system" << std::endl;

		const auto p_param = reinterpret_cast<risJobSystemParameters*>(param);

		instance_ = risJobSystem();
		instance_.threads_ = p_param->threads;
	}

	void risJobSystem::destroy()
	{
		std::cout << "destroy job system" << std::endl;
	}

	U32 risJobSystem::get_threads()
	{
		return threads_;
	}
}
