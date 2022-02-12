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
	int convert(const typename From::Character* input, typename To::Character* output, std::function<CodePoint(CodePoint)> replace_callback = nullptr)
	{
		I32 i = 0, j = 0;
	
		while (input[i] != 0)
		{
			auto input_lambda = [&]
			{
				return input[i++];
			};
			auto output_lambda = [&](typename To::Character c)
			{
				output[j++] = c;
			};
	
			const auto code_point = From::decode(input_lambda);
			if (replace_callback != nullptr)
				To::encode(replace_callback(code_point), output_lambda);
			else
				To::encode(code_point, output_lambda);
		}
	
		output[j] = 0;
		return j;
	}
}
