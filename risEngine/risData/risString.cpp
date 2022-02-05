#include "pch.h"
#include "risString.h"
#if defined _DEBUG
#include <map>
#endif

#include <string>

#include "../../3rd_party/stuff/crc32.h"
#include "risEncodings.h"

namespace risEngine
{
#pragma region global
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

	StringId sid(char* str)
	{
		return sid(static_cast<const char*>(str));
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

#pragma endregion

#pragma region risStringBuffer
#pragma region unformatted input
	template<typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::put(Character value)
	{
		if (position_ < memory_size_)
			memory_[position_++] = value;

		return *this;
	}

	template <typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::put(CodePoint code_point)
	{
		Encoding::encode(*this, code_point);

		return *this;
	}

	template<typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::put(const Character* values)
	{
		for (StreamSize i = 0; values[i] != 0; ++i)
		{
			put(values[i]);
		}

		return *this;
	}
#pragma endregion

#pragma region formatted input
	template<typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::format(bool value)
	{
		if (value)
		{
			put(static_cast<CodePoint>(116)); // t
			put(static_cast<CodePoint>(114)); // r
			put(static_cast<CodePoint>(117)); // u
			put(static_cast<CodePoint>(101)); // e
		}
		else
		{
			put(static_cast<CodePoint>(102)); // f
			put(static_cast<CodePoint>( 97)); // a
			put(static_cast<CodePoint>(108)); // l
			put(static_cast<CodePoint>(115)); // s
			put(static_cast<CodePoint>(101)); // e
		}

		return *this;
	}

	template <typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::format(I32 value)
	{
		constexpr CodePoint code_point[10]{ 48,49,50,51,52,53,54,55,56,57 };

		if (value == 0)
		{
			return put(static_cast<CodePoint>(code_point[0]));
		}

		if (value < 0)
		{
			put(static_cast<CodePoint>(45)); // -
			value *= -1;
		}

		U32 digit_count = 0;
		U32 value_copy = value;
		while (value_copy > 0)
		{
			++digit_count;
			value_copy /= 10;
		}

		for (U32 i = 0; i < digit_count; ++i)
		{
			const U32 digit_index = digit_count - i - 1;
			U32 div = 1;
			for (U32 j = 0; j < digit_index; ++j)
			{
				div *= 10;
			}

			const U32 digit_value = value / div % 10;

			put(static_cast<CodePoint>(code_point[digit_value]));
		}

		return *this;
	}

	template <typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::format(F32 value, U8 precision)
	{
		constexpr CodePoint code_point[10]{ 48,49,50,51,52,53,54,55,56,57 };

		I32 int_value = static_cast<I32>(value);

		format(int_value);
		put(static_cast<CodePoint>(46)); // .

		U32 mul = 1;
		for (U8 i = 0; i < precision; ++i)
		{
			mul *= 10;
		}

		int_value = static_cast<I32>((value - static_cast<F32>(int_value)) * static_cast<float>(mul));
		if (int_value < 0)
			int_value *= -1;

		for (U32 i = 0; i < precision; ++i)
		{
			const U32 digit_index = precision - i - 1;
			U32 div = 1;
			for (U32 j = 0; j < digit_index; ++j)
			{
				div *= 10;
			}

			const U32 digit_value = int_value / div % 10;

			put(static_cast<CodePoint>(code_point[digit_value]));
		}

		return *this;
	}
#pragma endregion


#pragma region stream utility
	template<typename Encoding>
	StreamPosition risStringBuffer<Encoding>::tellp() const
	{
		return position_;
	}

	template<typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::seekp(StreamPosition offset, StreamLocation stream_location)
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

	template<typename Encoding>
	risStringBuffer<Encoding>& risStringBuffer<Encoding>::flush()
	{
		return *this;
	}
#pragma endregion

#pragma region output
	template <typename Encoding>
	typename risStringBuffer<Encoding>::Character risStringBuffer<Encoding>::take()
	{
		if (position_ < memory_size_)
			return memory_[position_++];
		else
			return 0;
	}

	template<typename Encoding>
	void risStringBuffer<Encoding>::get_encoded_string(Character* buffer, StreamSize buffer_size)
	{
		put(static_cast<CodePoint>(0));
		for (StreamSize i = 0; i < buffer_size && i < memory_size_; ++i)
		{
			auto test = memory_[i];
			buffer[i] = test;
		}
	}

	template<typename Encoding>
	void risStringBuffer<Encoding>::get_decoded_string(CodePoint* buffer, StreamSize buffer_size)
	{
		put(static_cast<Character>(0));
		auto current_position = position_;

		seekp(0, StreamLocation::Beginning);

		for (StreamSize i = 0; position_ < current_position; ++i)
		{
			buffer[i] = Encoding::decode(*this);
		}

		position_ = current_position;
	}

	template <typename Encoding>
	typename risStringBuffer<Encoding>::Character* risStringBuffer<Encoding>::get_buffer()
	{
		return memory_;
	}
#pragma endregion

	template class risStringBuffer<risUTF8<>>;
	template class risStringBuffer<risASCII<>>;
#pragma endregion
}
