#include "pch.h"
#include "risString.h"
#if defined _DEBUG
#include <map>
#endif

#include <string>

#include "../risData/crc32.h"
#include "risMemoryUtility.h"

namespace risMemory
{
#if defined _DEBUG
	static std::map<StringId, const char*> gStringIdTable;
#endif

	StringId sid(const char* str)
	{
		const StringId string_id = crc32(str);

#if defined _DEBUG
		const auto it = gStringIdTable.find(string_id);
		if (it == gStringIdTable.end())
		{
			gStringIdTable[string_id] = _strdup(str);
		}
#endif

		return string_id;
	}

	const char* internal_string(StringId sid)
	{
#if defined _DEBUG
		const auto it = gStringIdTable.find(sid);
		return it != gStringIdTable.end()
			? it.operator*().second
			: nullptr;
#else
		return nullptr;
#endif
	}

	void risStringBuffer::init(U8* buffer, U32 buffer_size)
	{
		_buffer = buffer;
		_buffer_size = buffer_size;
		clear();
	}

	void risStringBuffer::clear()
	{
		init0(_buffer, _buffer_size);

		// for (U32 i = 0; i < _buffer_size; ++i)
		// {
		// 	_buffer[i] = 0;
		// }

		// *_buffer = {};

		_pointer = 0;
		_character_count = 0;
	}

	bool risStringBuffer::append(const char* s)
	{
		U32 input_size = 0;
		while (s[input_size] != 0)
		{
			++input_size;
		}

		if (_pointer + input_size >= _buffer_size)
			return false;

		for (U32 i = 0; i < input_size; ++i)
		{
			_buffer[_pointer++] = s[i];
			++_character_count;
		}

		return true;
	}

	bool risStringBuffer::append(U8 byte)
	{
		if (_pointer + 1 >= _buffer_size)
			return false;

		_buffer[_pointer++] = byte;
		++_character_count;
		return true;
	}

	bool risStringBuffer::append_utf8(U32 codepoint)
	{
		if (codepoint > 0x0010FFFF)
			return false;

		if (codepoint > 0x000FFFF)
		{
			if (_pointer + 4 >= _buffer_size)
				return false;

			_buffer[_pointer++] = static_cast<U8>(0xF0 | (0x1C0000 & codepoint) >> 18);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0x3F000 & codepoint) >> 12);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0xFC0 & codepoint) >> 6);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0x3F & codepoint));

			++_character_count;
			return true;
		}
		else if (codepoint > 0x000007FF)
		{
			if (_pointer + 3 >= _buffer_size)
				return false;

			_buffer[_pointer++] = static_cast<U8>(0xE0 | (0xF000 & codepoint) >> 12);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0xFC0 & codepoint) >> 6);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0x3F & codepoint));

			++_character_count;
			return true;
		}
		else if (codepoint > 0x0000007F)
		{
			if (_pointer + 2 >= _buffer_size)
				return false;

			_buffer[_pointer++] = static_cast<U8>(0xC0 | (0x7C0 & codepoint) >> 6);
			_buffer[_pointer++] = static_cast<U8>(0x80 | (0x3F & codepoint));

			++_character_count;
			return true;
		}
		else
		{
			if (_pointer + 1 >= _buffer_size)
				return false;

			_buffer[_pointer++] = static_cast<U8>(0x0000007f & codepoint);

			++_character_count;
			return true;
		}
	}

	void risStringBuffer::decode_utf8(U32* buffer)
	{
		U32 read_pointer = 0;

		const U32 count = character_count();
		for(U32 i = 0; i < count; ++i)
		{
			U8 byte1 = _buffer[read_pointer++];
			if ((byte1 & 0x80) == 0)
			{
				buffer[i] = byte1;
			}
			else if ((byte1 & 0xE0) == 0xC0)
			{
				U8 byte2 = _buffer[read_pointer++];

				buffer[i] = (byte1 & 0x1F) << 6 | byte2 & 0x3F;
			}
			else if ((byte1 & 0xF0) == 0xE0)
			{
				U8 byte2 = _buffer[read_pointer++];
				U8 byte3 = _buffer[read_pointer++];

				buffer[i] = (byte1 & 0x0F) << 12 | (byte2 & 0x3F) << 6 | byte3 & 0x3F;
			}
			else if ((byte1 & 0xF8) == 0xF0)
			{
				U8 byte2 = _buffer[read_pointer++];
				U8 byte3 = _buffer[read_pointer++];
				U8 byte4 = _buffer[read_pointer++];

				buffer[i] = (byte1 & 0x07) << 18 | (byte2 & 0x3F) << 12 | (byte3 & 0x3F) << 6 | byte4 & 0x3F;
			}
			else
			{
				// did not expect to find 0b10xxxxxx here
				// discarding character...
				--i;
			}
		}
	}

	U32 risStringBuffer::character_count()
	{
		return _character_count + 1;
	}

	U32 risStringBuffer::size()
	{
		return _pointer + 1;
	}

	U8* risStringBuffer::get_string()
	{
		return _buffer;
	}
}
