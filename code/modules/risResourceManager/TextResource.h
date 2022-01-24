#pragma once

#include "../risData/risPrimitives.h"
#include "../risData/risStreams.h"

namespace risResource
{
	using namespace risData;

	class TextResource
	{
	public:
		static TextResource parseFile(U8* data, StreamSize count);
		static TextResource parseData(U8* data, StreamSize count);
	};
}
