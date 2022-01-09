#include "pch.h"
#include "risString.h"
#if defined _DEBUG
#include <map>
#endif

#include <string>

#include "crc32.h"

namespace risData
{
#if defined _DEBUG
	static std::map<StringId, const char*> gStringIdTable;
#endif

	StringId internal_string_to_sid(const char* str)
	{
		StringId sid = crc32(str);

#if defined _DEBUG
		const auto it = gStringIdTable.find(sid);
		if (it == gStringIdTable.end())
		{
			gStringIdTable[sid] = _strdup(str);
		}
#endif

		return sid;
	}

	const char* sid_to_string(StringId sid)
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
