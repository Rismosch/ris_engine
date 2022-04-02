#pragma once
#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	typedef U32 StringId;
	extern StringId string_id(const char* str);
	extern const char* internal_string(StringId sid);
}
