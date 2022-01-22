#include "pch.h"
#include "risString.h"
#if defined _DEBUG
#include <map>
#endif

#include <string>

#include "crc32.h"
#include "risEncodings.h"

namespace risData
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

#pragma region risStringBuffer
	template<typename encoding>
	void risStringBuffer<encoding>::init(Character* memory, StreamSize memory_size)
	{
		memory_ = memory;
		memory_size_ = memory_size;
		position_ = 0;
	}

	template<typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::put(Character value)
	{
		if (position_ + 1 < memory_size_)
			memory_[position_++] = value;

		return *this;
	}

	template<typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::put(Character* values, StreamSize count)
	{
		for (StreamSize i = 0; i < count; ++i)
		{
			put(values[i]);
		}

		return *this;
	}

	template<typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::put(const Character* values)
	{
		for (StreamSize i = 0; values[i] != 0; ++i)
		{
			put(values[i]);
		}

		return *this;
	}

	template <typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::put(CodePoint code_point)
	{
		encoding::encode(*this, code_point);

		return *this;
	}

	template <typename encoding>
	typename risStringBuffer<encoding>::Character risStringBuffer<encoding>::take()
	{
		if (position_ < memory_size_)
			return memory_[position_++];
		else
			return 0;
	}


	template <typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::put(CodePoint* code_points, StreamSize count)
	{
		for (StreamSize i = 0; i < count; ++i)
		{
			put(code_points[i]);
		}
		
		return *this;
	}
	
	template<typename encoding>
	StreamPosition risStringBuffer<encoding>::tellp() const
	{
		return position_;
	}
	
	template<typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::seekp(StreamPosition offset, StreamLocation stream_location)
	{
		switch (stream_location)
		{
		case StreamLocation::Beginning:
			position_ = offset;
			break;
	
		case StreamLocation::Current:
			position_ += offset;
			break;
	
		case StreamLocation::End:
			position_ = memory_size_ + offset - 1;
			break;
		}
	
		return *this;
	}
	
	template<typename encoding>
	risStringBuffer<encoding>& risStringBuffer<encoding>::flush()
	{
		return *this;
	}

	template<typename encoding>
	void risStringBuffer<encoding>::get_encoded_string(Character* buffer, StreamSize buffer_size)
	{
		put(static_cast<Character>(0));
		for (StreamSize i = 0; i < buffer_size && i < memory_size_; ++i)
		{
			buffer[i] = memory_[i];
		}
	}

	template<typename encoding>
	void risStringBuffer<encoding>::get_decoded_string(CodePoint* buffer, StreamSize buffer_size)
	{
		put(static_cast<Character>(0));
		auto current_position = position_;

		seekp(0, StreamLocation::Beginning);

		for (StreamSize i = 0; position_ < current_position; ++i)
		{
			buffer[i] = encoding::decode(*this);
		}

		position_ = current_position;
	}

	template class risStringBuffer<risUTF8<>>;
	template class risStringBuffer<risASCII<>>;
#pragma endregion
}
