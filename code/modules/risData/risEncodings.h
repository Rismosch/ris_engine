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
		static void encode(OutputStream& output_stream, CodePoint code_point)
		{
			if (code_point > 0x0010FFFF)
				return;

			if (code_point > 0x000FFFF)
			{
				output_stream.put(static_cast<Character>(0xF0 | (0x1C0000 & code_point) >> 18));
				output_stream.put(static_cast<Character>(0x80 | (0x3F000 & code_point) >> 12));
				output_stream.put(static_cast<Character>(0x80 | (0xFC0 & code_point) >> 6));
				output_stream.put(static_cast<Character>(0x80 | (0x3F & code_point)));
			}
			else if (code_point > 0x000007FF)
			{
				output_stream.put(static_cast<Character>(0xE0 | (0xF000 & code_point) >> 12));
				output_stream.put(static_cast<Character>(0x80 | (0xFC0 & code_point) >> 6));
				output_stream.put(static_cast<Character>(0x80 | (0x3F & code_point)));
			}
			else if (code_point > 0x0000007F)
			{
				output_stream.put(static_cast<Character>(0xC0 | (0x7C0 & code_point) >> 6));
				output_stream.put(static_cast<Character>(0x80 | (0x3F & code_point)));
			}
			else
			{
				output_stream.put(static_cast<Character>(0x0000007f & code_point));
			}
		}

		template<typename InputStream>
		static CodePoint decode(InputStream& input_stream)
		{
			Character byte1 = input_stream.take();
			if ((byte1 & 0x80) == 0)
			{
				return static_cast<CodePoint>(byte1);
			}

			if ((byte1 & 0xE0) == 0xC0)
			{
				Character byte2 = input_stream.take();
				if ((byte2 & 0xC0) != 0x80)
					return 0xFFFF;

				return (byte1 & 0x1F) << 6 | byte2 & 0x3F;
			}
			
			if ((byte1 & 0xF0) == 0xE0)
			{
				Character byte2 = input_stream.take();
				if ((byte2 & 0xC0) != 0x80)
					return 0xFFFF;

				Character byte3 = input_stream.take();
				if ((byte3 & 0xC0) != 0x80)
					return 0xFFFF;

				return (byte1 & 0x0F) << 12 | (byte2 & 0x3F) << 6 | byte3 & 0x3F;
			}

			if ((byte1 & 0xF8) == 0xF0)
			{
				Character byte2 = input_stream.take();
				if ((byte2 & 0xC0) != 0x80)
					return 0xFFFF;

				Character byte3 = input_stream.take();
				if ((byte3 & 0xC0) != 0x80)
					return 0xFFFF;

				Character byte4 = input_stream.take();
				if ((byte4 & 0xC0) != 0x80)
					return 0xFFFF;

				return (byte1 & 0x07) << 18 | (byte2 & 0x3F) << 12 | (byte3 & 0x3F) << 6 | byte4 & 0x3F;
			}

			return 0xFFFF;
		}
	};

	template<typename CharType = char>
	struct risASCII
	{
		typedef CharType Character;

		template<typename OutputStream>
		static void encode(OutputStream& output_stream, CodePoint code_point)
		{
			output_stream.put(static_cast<Character>(code_point & 0x7F));
		}

		template<typename InputStream>
		static CodePoint decode(InputStream& input_stream)
		{
			return input_stream.take() & 0x7F;
		}
	};
}
