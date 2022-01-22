#include "pch.h"
#include "risDataUtility.h"
#include <cstring>

namespace risData
{
	void init0(void* dest, U32 count)
	{
		std::memset(dest, 0, count);
	}
}
