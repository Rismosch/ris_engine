#pragma once
#include <functional>

#include "risPrimitives.h"

namespace risEngine
{
	typedef U32 CodePoint;
	
	// official UTF8 standard: https://datatracker.ietf.org/doc/html/rfc3629
	// official UTF16 standard: https://datatracker.ietf.org/doc/html/rfc2781

	struct risUtf8
	{
		// encoding policy
		typedef char Character;
		static void encode(CodePoint input, std::function<void (Character)> output);
		static CodePoint decode(std::function<Character()> input);
	};

	struct risUtf16LE
	{
		// encoding policy
		typedef wchar_t Character;
		static void encode(CodePoint input, std::function<void(Character)> output);
		static CodePoint decode(std::function<Character()> input);
	};
}
