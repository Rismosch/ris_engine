#include "pch.h"
#include "FlagModule.h"

namespace risUtility
{
	struct FlagModule::Impl
	{
		constexpr static U8 flag_count = 64;
		U64* flags = nullptr;

		Impl() : flags(new U64(0)) { }
		~Impl() { delete flags; }

		constexpr static U64 mask(U8 flag)
		{
			return static_cast<U64>(1) << flag;
		}
	};

	FlagModule::FlagModule() : pImpl(new Impl()) { }
	FlagModule::~FlagModule() { delete pImpl; }

	bool FlagModule::get(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return false;

		const auto value = *pImpl->flags;
		const auto mask = Impl::mask(flag);

		return (value & mask) != 0;
	}

	void FlagModule::set(U8 flag, bool value) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = Impl::mask(flag);

		if (value)
			*pImpl->flags |= mask;
		else
			*pImpl->flags &= ~mask;
	}

	void FlagModule::toggle(U8 flag) const
	{
		if (flag >= Impl::flag_count)
			return;

		const auto mask = Impl::mask(flag);
		*pImpl->flags ^= mask;
	}

	std::string FlagModule::toString() const
	{
		constexpr U8 groupBy = 8;

		std::string result;
		for (U8 i = 0; i < Impl::flag_count; ++i)
		{
			if (i != 0 && i % groupBy == 0)
				result.append(" ");

			result.append(get(i) ? "1" : "0");
		}

		return result;
	}
}
