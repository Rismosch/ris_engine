#pragma once
#include <cstdint>
#include "../risData/risPrimitives.h"

namespace risEngine
{
	struct risJobSystemParameters
	{
		U32 threads;
	};

	class risJobSystem
	{
	public:
		// singleton policy
		static risJobSystem* instance();
		static void create(uintptr_t param);
		static void destroy();

		risJobSystem(const risJobSystem& other) = delete;
		risJobSystem(risJobSystem && other) noexcept = delete;
	private:
		risJobSystem() = default;
		static risJobSystem* p_instance_;
	};
}
