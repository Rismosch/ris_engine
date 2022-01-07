#include "pch.h"
#include "stringid.h"
#include <map>

#include "crc32.h"

namespace risData
{
	static std::map<StringId, const char*> gStringIdTable;

	StringId risStringToSid(const char* str)
	{
		StringId sid = crc32(str);

		const auto it = gStringIdTable.find(sid);
		if (it == gStringIdTable.end())
		{
			gStringIdTable[sid] = _strdup(str);
		}
		
		return sid;
	}

	const char* risSidToString(StringId sid)
	{
		const auto it = gStringIdTable.find(sid);
		return it != gStringIdTable.end()
			? it.operator*().second
			: nullptr;
	}

}
