#pragma once
#include "../risData/risPrimitives.h"

namespace risData
{
	// adapted from https://web.archive.org/web/20190108202303/http://www.hackersdelight.org/hdcodetxt/crc.c.txt
	inline U32 crc32(const char* message)
	{
		I32 i, j;
		U32 byte, crc, mask;

		i = 0;
		crc = 0xFFFFFFFF;
		while (message[i] != 0)
		{
			byte = static_cast<unsigned char>(message[i]);	// Get next byte.
			crc = crc ^ byte;
			for (j = 7; j >= 0; --j)
			{
				mask = 0 - (crc & 1);
				crc = (crc >> 1) ^ (0xEDB88320 & mask);
			}
			i = i + 1;
		}
		return ~crc;
	}
}
