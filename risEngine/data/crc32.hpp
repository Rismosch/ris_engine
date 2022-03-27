#include "pch.h"
#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	// https://web.archive.org/web/20190108202303/http://www.hackersdelight.org/hdcodetxt/crc.c.txt
	inline U32 crc32(const char* message)
	{
		I32 i = 0;
		U32 crc = 0xFFFFFFFF;
		while (message[i] != 0)
		{
			const U32 byte = static_cast<U32>(message[i]);	// Get next byte.
			crc = crc ^ byte;
			for (I32 j = 7; j >= 0; --j)
			{
				const U32 mask = 0 - (crc & 1);
				crc = (crc >> 1) ^ (0xEDB88320 & mask);
			}
			i = i + 1;
		}
		return ~crc;
	}
}
