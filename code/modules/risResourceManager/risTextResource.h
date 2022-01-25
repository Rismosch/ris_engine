#pragma once

#include "../risData/risPrimitives.h"

namespace risResource
{
	using namespace risData;

	class risTextResource
	{
	public:
		static risTextResource parseFile(U8* data, U32 count);
		static risTextResource parseData(U8* data, U32 count);
	};
}
