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

		risStringBuffer& put(Character value);
		risStringBuffer& put(Character* values, StreamSize count);
		risStringBuffer& put(const Character* values);
		risStringBuffer& put(CodePoint code_point);
		risStringBuffer& put(CodePoint* code_points, StreamSize count);

		Character take();

		StreamPosition tellp() const;
		risStringBuffer& seekp(StreamPosition offset, StreamLocation stream_location = StreamLocation::Beginning);
		risStringBuffer& flush();

		void get_encoded_string(Character* buffer, StreamSize buffer_size);
		void get_decoded_string(CodePoint* buffer, StreamSize buffer_size);

	private:
		Character* memory_;
		StreamSize memory_size_;

		StreamSize pointer_;
	};

	typedef risStringBuffer<risUTF8<>> risStringUTF8;
	typedef risStringBuffer<risASCII<>> risStringASCII;
}
