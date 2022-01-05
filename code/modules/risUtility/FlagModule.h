#pragma once
#include <iostream>

namespace risFlag
{
	// Flags
	constexpr auto TEST_START_LINE = __LINE__;
	enum class flag
	{
		Test0,
		Test1,
		Test2,
		Test3
	};
	constexpr auto flag_count = __LINE__ - TEST_START_LINE - 4;

	class FlagModule
	{
	public:
		FlagModule();
		~FlagModule();

		bool get(flag flag) const;
		void set(flag flag, bool value) const;
		void toggle(flag flag) const;

		std::string to_string(int group = 8) const;
	private:
		struct Impl;
		Impl* pImpl;
	};
}
