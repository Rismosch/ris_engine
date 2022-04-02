#pragma once
#include <risEngine/data/primitives.hpp>
#include <cstdint>

namespace risEngine
{
	struct risJobSystemParameters
	{
		U32 threads;
	};

	class risJobSystem
	{
	public: // singleton policy
		static risJobSystem& instance();
		static void create(uintptr_t param);
		static void destroy();

	private:
		static risJobSystem instance_;

	public: // Public Methods
		U32 get_threads();

	private:
		U32 threads_ = 0;
	};
}
