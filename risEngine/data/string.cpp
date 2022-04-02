#include "pch.h"
#include <risEngine/data/string.hpp>
#include <risEngine/data/crc32.hpp>

#include <string>
#if defined _DEBUG
#include <map>
#endif

namespace risEngine
{
#if defined _DEBUG
	static std::map<StringId, const char*> gStringIdTable;
#endif

	StringId string_id(const char* str)
	{
		const StringId string_id = crc32(str);

#if defined _DEBUG
		const auto it = gStringIdTable.find(string_id);
		if (it == gStringIdTable.end())
		{
			gStringIdTable[string_id] = _strdup(str);
		}
#endif

		return string_id;
	}

	const char* internal_string(StringId sid)
	{
#if defined _DEBUG
		const auto it = gStringIdTable.find(sid);
		return it != gStringIdTable.end()
			? it.operator*().second
			: nullptr;
#else
		return nullptr;
#endif
	}
}
