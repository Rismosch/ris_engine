#include "pch.h"
#include "FlagModule.h"

namespace risFlag
{
	struct FlagModule::Impl
	{
		int* flags = nullptr;

		Impl() : flags(new int[flag_count])
		{
			for (int i = 0; i < flag_count; ++i)
			{
				flags[i] = 0;
			}
		}

		~Impl()
		{
			delete[] flags;
		}

		static int page(flag flag)
		{
			constexpr int page_size = sizeof(int) * 8;

			int result = 0;
			auto value = static_cast<int>(flag);

			while (value >= page_size)
			{
				value -= page_size;
				++result;
			}

			return result;
		}

		static int mask(flag flag)
		{
			constexpr int page_size = sizeof(int) * 8;

			return 1 << (static_cast<int>(flag) % page_size);
		}
	};

	FlagModule::FlagModule() : pImpl(new Impl()) { }
	FlagModule::~FlagModule() { delete pImpl; }

	bool FlagModule::get(flag flag) const
	{
		const int value = pImpl->flags[Impl::page(flag)];
		const int mask = Impl::mask(flag);

		return (value & mask) != 0;
	}

	void FlagModule::set(flag flag, bool value) const
	{
		const int mask = Impl::mask(flag);

		if (value)
			pImpl->flags[Impl::page(flag)] |= mask;
		else
			pImpl->flags[Impl::page(flag)] &= ~mask;
	}

	void FlagModule::toggle(flag flag) const
	{
		const int mask = Impl::mask(flag);
		pImpl->flags[Impl::page(flag)] ^= mask;
	}

	std::string FlagModule::to_string(int group) const
	{
		std::string result;
		for (int i = 0; i < flag_count; ++i)
		{
			if (i != 0 && i % group == 0)
				result.append(" ");

			const auto flag = static_cast<risFlag::flag>(i);
			const bool value = get(flag);

			result.append(value ? "1" : "0");
		}

		return result;
	}
}
