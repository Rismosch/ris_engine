#pragma once
#include <iostream>
#include "../risData/risData.h"

namespace risUtility
{
	using namespace risData;
	
	class risFlag
	{
	public:
		typedef U32 FlagCollection;

		risFlag();
		~risFlag();
		
		void apply(FlagCollection flags) const;
		FlagCollection retrieve() const;
		
		bool get(U8 flag) const;
		void set(U8 flag, bool value) const;
		void toggle(U8 flag) const;

		std::string toString() const;
	private:
		struct Impl;
		Impl* pImpl;
	};
}
