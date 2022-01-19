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
		virtual ~risOutStream() = default;
		virtual risOutStream& put(C8 value) = 0;
		virtual risOutStream& write(const C8* values, U32 count) = 0;
		virtual I64 tellp() = 0;
		virtual risOutStream& seekp(I64 offset, StreamPosition stream_position) = 0;
		virtual risOutStream& flush() = 0;
	};
}