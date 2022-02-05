#pragma once

#include <fstream>

namespace risEngine
{
	inline bool file_exists(const char* filename)
	{
		const std::ifstream f(filename);
		return f.good();
	}
}
