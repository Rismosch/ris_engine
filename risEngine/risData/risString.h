#pragma once
#include "risEncodings.h"

namespace risEngine
{
	typedef U32 StringId;
	extern StringId sid(const char* str);
	extern StringId sid(char* str);
	extern const char* internal_string(StringId sid);
}
