#include "pch.h"
#include "risFlag.h"

namespace risEngine
{
	void risFlag::apply(FlagCollection flags)
	{
		flags_ = flags;
	}

	risFlag::FlagCollection risFlag::retrieve() const
	{
		return flags_;
	}

	bool risFlag::get(Flag flag) const
	{
		if (flag >= flag_count_)
			return false;

		const auto value = flags_;
		const auto mask = static_cast<FlagCollection>(1) << flag;

		return (value & mask) != 0;
	}

	void risFlag::set(Flag flag, bool value)
	{
		if (flag >= flag_count_)
			return;

		const auto mask = static_cast<FlagCollection>(1) << flag;

		if (value)
			flags_ |= mask;
		else
			flags_ &= ~mask;
	}

	void risFlag::toggle(Flag flag)
	{
		if (flag >= flag_count_)
			return;

		const auto mask = static_cast<FlagCollection>(1) << flag;
		flags_ ^= mask;
	}

#if defined _DEBUG
	const char* risFlag::to_string()
	{
		constexpr U8 group_by = 8;
		constexpr U8 number_spaces = flag_count_ / group_by - (flag_count_ % group_by == 0) + 1;
		constexpr U8 string_length = flag_count_ + number_spaces;

		char* result = new char[string_length];

		for (U8 i = 0, j = string_length - 1; i < flag_count_; ++i)
		{
			if (i == 0)
				result[j--] = '\0';
			else if (i % group_by == 0)
				result[j--] = ' ';
		
			result[j--] = get(i) ? '1' : '0';
		}

		return result;
	}
#endif
}
