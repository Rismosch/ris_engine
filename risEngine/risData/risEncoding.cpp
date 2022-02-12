#include "pch.h"

#include "risEncodings.h"

namespace risEngine
{
	void risUtf8::encode(const CodePoint input, const std::function<void(Character)>& output)
	{
		if (input > 0x0010FFFF)
			return;
		
		if (input > 0x000FFFF)
		{
			output(static_cast<Character>(0xF0 | (0x1C0000 & input) >> 18));
			output(static_cast<Character>(0x80 | (0x3F000 & input) >> 12));
			output(static_cast<Character>(0x80 | (0xFC0 & input) >> 6));
			output(static_cast<Character>(0x80 | (0x3F & input)));
		}
		else if (input > 0x000007FF)
		{
			output(static_cast<Character>(0xE0 | (0xF000 & input) >> 12));
			output(static_cast<Character>(0x80 | (0xFC0 & input) >> 6));
			output(static_cast<Character>(0x80 | (0x3F & input)));
		}
		else if (input > 0x0000007F)
		{
			output(static_cast<Character>(0xC0 | (0x7C0 & input) >> 6));
			output(static_cast<Character>(0x80 | (0x3F & input)));
		}
		else
		{
			output(static_cast<Character>(0x0000007f & input));
		}
	}

	CodePoint risUtf8::decode(const std::function<Character()>& input)
	{
		Character byte1 = input();
		if ((byte1 & 0x80) == 0)
		{
			return static_cast<CodePoint>(byte1);
		}

		if ((byte1 & 0xE0) == 0xC0)
		{
			Character byte2 = input();
			if ((byte2 & 0xC0) != 0x80)
				return 0xFFFF;

			return (byte1 & 0x1F) << 6 | byte2 & 0x3F;
		}

		if ((byte1 & 0xF0) == 0xE0)
		{
			Character byte2 = input();
			if ((byte2 & 0xC0) != 0x80)
				return 0xFFFF;

			Character byte3 = input();
			if ((byte3 & 0xC0) != 0x80)
				return 0xFFFF;

			return (byte1 & 0x0F) << 12 | (byte2 & 0x3F) << 6 | byte3 & 0x3F;
		}

		if ((byte1 & 0xF8) == 0xF0)
		{
			Character byte2 = input();
			if ((byte2 & 0xC0) != 0x80)
				return 0xFFFF;

			Character byte3 = input();
			if ((byte3 & 0xC0) != 0x80)
				return 0xFFFF;

			Character byte4 = input();
			if ((byte4 & 0xC0) != 0x80)
				return 0xFFFF;

			return (byte1 & 0x07) << 18 | (byte2 & 0x3F) << 12 | (byte3 & 0x3F) << 6 | byte4 & 0x3F;
		}

		return 0xFFFF;
	}

	void risUtf16LE::encode(const CodePoint input, const std::function<void(Character)>& output)
	{
		if (input < 0x10000)
		{
			output(static_cast<Character>(input));
		}
		else
		{
			const CodePoint shifted_code_point = input - 0x10000;

			output(static_cast<Character>(0xD800 | (0xFFC00 & shifted_code_point) >> 10));
			output(static_cast<Character>(0xDC00 | (0x3FF & shifted_code_point)));
		}
	}

	CodePoint risUtf16LE::decode(const std::function<Character()>& input)
	{
		Character w1 = input();
		if (w1 < 0xD800 || w1 > 0xDFFF)
			return w1;

		if (w1 <= 0xD800 || w1 >= 0xDBFF)
			return 0xFFFF;

		Character w2 = input();
		if (w2 == 0)
			return 0xFFFF;

		return (((w1 & 0x3FF) << 10) | (w2 & 0x3FF)) + 0x10000;
	}
}
