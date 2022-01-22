#pragma once
#include "risPrimitives.h"

namespace risData
{
	typedef U32 CodePoint;

	template<typename CharType = U8>
	struct risUTF8
	{
		typedef CharType Character;

		template<typename OutputStream>
		static void Encode(OutputStream& output_stream, CodePoint code_point)
		{
			if (code_point > 0x0010FFFF)
				return;

			if (code_point > 0x000FFFF)
			{
				output_stream.put_byte(static_cast<U8>(0xF0 | (0x1C0000 & code_point) >> 18));
				output_stream.put_byte(static_cast<U8>(0x80 | (0x3F000 & code_point) >> 12));
				output_stream.put_byte(static_cast<U8>(0x80 | (0xFC0 & code_point) >> 6));
				output_stream.put_byte(static_cast<U8>(0x80 | (0x3F & code_point)));
			}
			else if (code_point > 0x000007FF)
			{
				output_stream.put_byte(static_cast<U8>(0xE0 | (0xF000 & code_point) >> 12));
				output_stream.put_byte(static_cast<U8>(0x80 | (0xFC0 & code_point) >> 6));
				output_stream.put_byte(static_cast<U8>(0x80 | (0x3F & code_point)));
			}
			else if (code_point > 0x000007FF)
			{
				output_stream.put_byte(static_cast<U8>(0xC0 | (0x7C0 & code_point) >> 6));
				output_stream.put_byte(static_cast<U8>(0x80 | (0x3F & code_point)));
			}
			else
			{
				output_stream.put_byte(static_cast<U8>(0x0000007f & code_point));
			}
		}

		template<typename InputStream>
		static CodePoint Decode(InputStream& input_stream)
		{
			return 42;
		}
	};
}
