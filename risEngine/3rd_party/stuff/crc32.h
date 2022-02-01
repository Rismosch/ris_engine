#pragma once
#include <cstdint>

namespace risEngine
{
	// adapted from https://web.archive.org/web/20190108202303/http://www.hackersdelight.org/hdcodetxt/crc.c.txt
	inline uint32_t crc32(const char* message)
	{
		int32_t i = 0;
		uint32_t crc = 0xFFFFFFFF;
		while (message[i] != 0)
		{
			const uint32_t byte = static_cast<unsigned char>(message[i]);	// Get next byte.
			crc = crc ^ byte;
			for (int32_t j = 7; j >= 0; --j)
			{
				const uint32_t mask = 0 - (crc & 1);
				crc = (crc >> 1) ^ (0xEDB88320 & mask);
			}
			i = i + 1;
		}
		return ~crc;
	}
}