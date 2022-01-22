#pragma once
#include "risPrimitives.h"
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

		void init(U8* memory, StreamSize memory_size)
		{
			memory_ = memory;
			memory_size_ = memory_size;
		}

		risStringBuffer& put_byte(U8 value)
		{
			if (pointer_ + 1 < memory_size_)
				memory_[pointer_++] = value;

			return *this;
		}

		risStringBuffer& put(Character value)
		{
			encoding::encode(this, value);

			return *this;
		}
		risStringBuffer& write(Character* values, StreamSize count)
		{
			for (StreamSize i = 0; i < count; ++i)
			{
				put(values[i]);
			}

			return *this;
		}

		StreamPosition tellp() const
		{
			return pointer_;
		}

		risStringBuffer& seekp(StreamPosition offset, StreamLocation stream_location = StreamLocation::Beginning)
		{
			switch (stream_location)
			{
			case StreamLocation::Beginning:
				pointer_ = offset;
				break;

			case StreamLocation::Current:
				pointer_ += offset;
				break;

			case StreamLocation::End:
				pointer_ = memory_size_ + offset - 1;
				break;
			}

			return *this;
		}
		risStringBuffer& flush();


		void get_string(Character* buffer, StreamSize buffer_size)
		{
			
		}

	private:
		U8* memory_ = nullptr;
		StreamSize memory_size_ = 0;

		StreamSize pointer_ = 0;

	};
}
