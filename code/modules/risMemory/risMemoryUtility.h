#pragma once
#include "../risData/risPrimitives.h"
#include <cstring>

namespace risMemory
{
	using namespace risData;

	inline void init0(void* dest, U32 count)
	{
		std::memset(dest, 0, count);
	};
}
