#pragma once
#include "../risData/risData.h"

namespace risMemory
{
	using namespace risData;

	typedef U32 StringId;
	extern StringId sid(const char* str);
	extern const char* internal_string(StringId sid);

	class risStringBuffer
	{
	public:
		void init(U8* buffer, U32 buffer_size);

		void clear();

		bool append(const char* s);
		bool append(U8 byte);

		bool append_utf8(U32 codepoint);
		void decode_utf8(U32* buffer);

		U32 character_count();
		U32 size();

		U8* get_string();

	private:
		U8* _buffer = nullptr;
		U32 _buffer_size = 0;

		U32 _pointer = 0;
		U32 _character_count = 0;

	};
}
