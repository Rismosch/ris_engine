#pragma once
#include "../risData/risData.h"
#include <cstring>

namespace risUtility
{
	using namespace risData;

	constexpr void init0(void* dest, U32 count)
	{
		std::memset(dest, 0, count);
	};
}
