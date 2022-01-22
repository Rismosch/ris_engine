#include "pch.h"
#include "risFlag.h"

namespace risData
{
	struct risFlag::Impl
	{
		constexpr static auto flag_count = sizeof(FlagCollection) * 8;
		FlagCollection* flags = nullptr;

		Impl() : flags(new FlagCollection(0)) { }
		~Impl() { delete flags; }
	};

	risFlag::risFlag() : pImpl(new Impl()) { }
	risFlag::~risFlag() { delete pImpl; }

	void risFlag::apply(FlagCollection flags) const
	{
		*pImpl->flags = flags;
	}

	risFlag::FlagCollection risFlag::retrieve() const
	{
		return *pImpl->flags;
	}

	bool risFlag::get(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return false;

		const auto value = *pImpl->flags;
		const auto mask = static_cast<FlagCollection>(1) << flag;

		return (value & mask) != 0;
	}

	void risFlag::set(U8 flag, bool value) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = static_cast<FlagCollection>(1) << flag;

		if (value)
			*pImpl->flags |= mask;
		else
			*pImpl->flags &= ~mask;
	}

	void risFlag::toggle(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = static_cast<FlagCollection>(1) << flag;
		*pImpl->flags ^= mask;
	}

	const U8* risFlag::to_string() const
	{
		constexpr U8 group_by = 8;
		constexpr U8 number_spaces = Impl::flag_count / group_by - (Impl::flag_count % group_by == 0) + 1;
		constexpr U8 string_length = Impl::flag_count + number_spaces;

		U8* result = new U8[string_length];

		for (U8 i = 0, j = string_length - 1; i < Impl::flag_count; ++i)
		{
			if (i == 0)
				result[j--] = '\0';
			else if (i % group_by == 0)
				result[j--] = ' ';
		
			result[j--] = get(i) ? '1' : '0';
		}

		return result;
	}
}
