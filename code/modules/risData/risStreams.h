#pragma once
#include "pch.h"

#include "../risData/risPrimitives.h"

#define EndOfFile (-1)

namespace risData
{
	typedef I64 StreamPosition;
	typedef I64 StreamSize;
	typedef I32 StreamCharacter;

	enum class StreamLocation
	{
		Beginning,
		Current,
		End
	};

	// after some consolidation, mabye I should use templates instead of inheritance
	// class risOutStream
	// {
	// public:
	// 	virtual ~risOutStream() = default;
	// 	virtual risOutStream& put(char value) = 0;
	// 	virtual risOutStream& write(const char* values, StreamSize count) = 0;
	//
	// 	virtual StreamPosition tellp() = 0;
	// 	virtual risOutStream& seekp(StreamPosition offset, StreamLocation stream_location) = 0;
	// 	virtual risOutStream& flush() = 0;
	// };
	//
	// class risInStream
	// {
	// public:
	// 	virtual ~risInStream() = default;
	// 	virtual StreamSize gcount() = 0;
	//
	// 	virtual risInStream& get(char* buffer, StreamSize count) = 0;
	// 	virtual risInStream& get(char* buffer, StreamSize count, char delim) = 0;
	// 	virtual risInStream& get(risOutStream& buffer, StreamSize count) = 0;
	// 	virtual risInStream& get(risOutStream& buffer, StreamSize count, char delim) = 0;
	//
	// 	virtual risInStream& ignore(StreamSize count = 1, StreamCharacter delim = EndOfFile) = 0;
	//
	// 	virtual StreamPosition tellg() = 0;
	// 	virtual risInStream& seekg(StreamPosition offset, StreamLocation stream_location) = 0;
	// 	virtual I32 sync() = 0;
	// };
}