#pragma once
#include "../risData/risData.h"
#include <cstring>

namespace risMemory
{
	using namespace risData;

	void init0(void* dest, U32 count)
	{
		std::memset(dest, 0, count);
	};
}
