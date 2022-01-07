#include "pch.h"
#include "stringid.h"
#include <map>

#include "crc32.h"

namespace risData
{
	static std::map<StringId, const char*> gStringIdTable;

	StringId internString(const char* str)
	{
		StringId sid = crc32(str);
		
		auto it = gStringIdTable.find(sid);
		
		if (it == gStringIdTable.end())
		{
			gStringIdTable[sid] = _strdup(str);
		}
		
		return sid;
	}

}
