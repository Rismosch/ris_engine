#pragma once
#include "risEncodings.h"
#include "risStreams.h"

namespace risEngine
{
	typedef U32 StringId;
	extern StringId sid(const char* str);
	extern StringId sid(char* str);
	extern const char* internal_string(StringId sid);

	template<typename Encoding>
	class risStringBuffer
	{
	public:
		typedef typename Encoding::Character Character;

		risStringBuffer() = default;

		template<class Allocator>
		risStringBuffer(Allocator* allocator, StreamSize memory_size)
		{
			memory_ = static_cast<Character*>(allocator->alloc(memory_size));
			memory_size_ = memory_size;
			position_ = 0;
		}

		// unformatted input
		risStringBuffer& put(Character value);
		risStringBuffer& put(CodePoint code_point);
		risStringBuffer& put(const Character* values);

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

		Character* get_buffer();

	private:
		Character* memory_ = nullptr;
		StreamSize memory_size_ = 0;

		StreamSize position_ = 0;
	};

	typedef risStringBuffer<risUTF8<>> risStringUTF8;
	typedef risStringBuffer<risASCII<>> risStringASCII;
	typedef risStringBuffer<risNoEncoding<>> risStringNoEncoding;
}
