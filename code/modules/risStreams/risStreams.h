#pragma once
#include "pch.h"

#include "../risData/risPrimitives.h"

#define EndOfFile (-1)

namespace risStreams
{
	using namespace risData;

	typedef I64 StreamPosition;
	typedef I64 StreamSize;
	typedef I32 StreamCharacter;

	enum class StreamLocation
	{
		Beginning,
		Current,
		End
	};
	
	class risOutStream
	{
	public:
		virtual ~risOutStream() = default;
		virtual risOutStream& put(char value) = 0;
		virtual risOutStream& write(const char* values, StreamSize count) = 0;
		virtual StreamPosition tellp() = 0;
		virtual risOutStream& seekp(StreamPosition offset, StreamLocation stream_location) = 0;
		virtual risOutStream& flush() = 0;
	};

	class risInStream
	{
	public:
		virtual ~risInStream() = default;
		virtual I64 gcount() = 0;
		// get
		virtual risInStream& getLine(char* buffer, StreamSize count, char delim = EndOfFile) = 0;
		virtual risInStream& ignore(StreamSize coutn = 1, StreamCharacter delim = EndOfFile) = 0;
		virtual I32 peek() = 0;
		virtual risInStream& read(char* buffer, StreamSize count) = 0;
		virtual StreamSize readsome(char* buffer, StreamSize count) = 0;
		virtual risInStream& putback(char character) = 0;
		virtual risInStream& unget() = 0;
		virtual StreamPosition tellp() = 0;
		virtual risInStream& seekp(StreamPosition offset, StreamLocation stream_location) = 0;
		virtual I32 sync() = 0;
	};
}