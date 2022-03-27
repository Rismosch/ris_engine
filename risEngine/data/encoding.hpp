#pragma once
#include <functional>

#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	typedef U32 CodePoint;

	// official UTF8 standard: https://datatracker.ietf.org/doc/html/rfc3629
	// official UTF16 standard: https://datatracker.ietf.org/doc/html/rfc2781
	
	struct risUtf8
	{
		// encoding policy
		typedef char Character;
		static void encode(CodePoint input, const std::function<void(Character)>& output);
		static CodePoint decode(const std::function<Character()>& input);
	};

	struct risUtf16LE
	{
		// encoding policy
		typedef wchar_t Character;
		static void encode(CodePoint input, const std::function<void(Character)>& output);
		static CodePoint decode(const std::function<Character()>& input);
	};

	template<class From, class To>
	I32 convert(const typename From::Character * input, typename To::Character * output, std::function<CodePoint(CodePoint)> replace_callback = nullptr)
	{
		I32 i = 0, j = 0;
	
		while (input[i] != 0)
		{
			auto code_point = From::decode([&] {return input[i++]; });

			if (replace_callback != nullptr)
				code_point = replace_callback(code_point);
			
			To::encode(code_point, [&](typename To::Character c) {output[j++] = c; });
		}
	
		output[j] = 0;
		return j;
	}
}
