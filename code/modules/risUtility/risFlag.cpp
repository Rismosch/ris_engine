#include "pch.h"
#include "risFlag.h"

namespace risUtility
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

	std::string risFlag::toString() const
	{
		constexpr U8 groupBy = 8;

		std::string result;
		for (U8 i = Impl::flag_count; i > 0; --i)
		{
			if (i != Impl::flag_count && i % groupBy == 0)
				result.append(" ");

			result.append(get(i - 1) ? "1" : "0");
		}

		return result;
	}
}
