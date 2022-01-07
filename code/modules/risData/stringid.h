#pragma once
#include "risData.h"

namespace risData
{
	typedef U32 StringId;
	extern StringId risStringToSid(const char* str);
	extern const char* risSidToString(StringId sid);
}
