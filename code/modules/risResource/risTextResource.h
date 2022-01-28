#pragma once

#include "../risData/risPrimitives.h"

namespace risResource
{
	using namespace risData;

	class risTextResource
	{
	public:
#if defined _DEBUG
		static risTextResource parseFile(U8* data, U32 count);
#endif
		static risTextResource parseData(U8* data, U32 count);
	};
}
