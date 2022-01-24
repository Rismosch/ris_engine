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
}