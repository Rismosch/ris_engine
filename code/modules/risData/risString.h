#pragma once
#include "risData.h"

namespace risData
{
	typedef U32 StringId;
	extern StringId sid(const char* str);
	extern const char* internal_string(StringId sid);
}
