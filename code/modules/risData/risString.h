#pragma once
#include "risData.h"

namespace risData
{
	typedef U32 StringId;
	extern StringId internal_string_to_sid(const char* str);
	extern const char* sid_to_string(StringId sid);
}
