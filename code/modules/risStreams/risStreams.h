#include "pch.h"

#include "../risData/risPrimitives.h"

namespace risStreams
{
	using namespace risData;

	enum class StreamPosition
	{
		Beginning,
		Current,
		End
	};
	
	class risOutStream
	{
	public:
		virtual risOutStream& put(U8 value) = 0;
		virtual risOutStream& write(U8* values, U32 count) = 0;
		virtual I32 tellp() = 0;
		virtual risOutStream& seekp(I32 offset, StreamPosition stream_position) = 0;
		virtual risOutStream& flush() = 0;
	};
}