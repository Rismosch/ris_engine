#pragma once
#include "risEncodings.h"
#include "risStreams.h"

namespace risData
{
	typedef U32 StringId;
	extern StringId sid(const char* str);
	extern const char* internal_string(StringId sid);

	template<typename encoding>
	class risStringBuffer
	{
	public:
		typedef typename encoding::Character Character;

		void init(Character* memory, StreamSize memory_size);

		// unformatted input
		risStringBuffer& put(Character value);
		risStringBuffer& put(Character* values, StreamSize count);
		risStringBuffer& put(const Character* values);
		risStringBuffer& put(CodePoint code_point);
		risStringBuffer& put(CodePoint* code_points, StreamSize count);

		// formatted input
		risStringBuffer& format(bool value);
		risStringBuffer& format(I32 value);
		risStringBuffer& format(F32 value, U8 precision = 8);

		// stream utility
		StreamPosition tellp() const;
		risStringBuffer& seekp(StreamPosition offset, StreamLocation stream_location = StreamLocation::Beginning);
		risStringBuffer& flush();

		// output
		Character take();
		void get_encoded_string(Character* buffer, StreamSize buffer_size);
		void get_decoded_string(CodePoint* buffer, StreamSize buffer_size);

	private:
		Character* memory_ = nullptr;
		StreamSize memory_size_ = 0;

		StreamSize position_ = 0;
	};

	typedef risStringBuffer<risUTF8<>> risStringUTF8;
	typedef risStringBuffer<risASCII<>> risStringASCII;
}
