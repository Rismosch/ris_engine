#pragma once
#include "../risData/risPrimitives.h"

namespace risUtility
{
	using namespace risData;
	
	class risFlag
	{
	public:
		typedef U64 FlagCollection;

		risFlag();
		~risFlag();
		
		void apply(FlagCollection flags) const;
		FlagCollection retrieve() const;
		
		bool get(U8 flag) const;
		void set(U8 flag, bool value) const;
		void toggle(U8 flag) const;

		const U8* to_string() const;
	private:
		struct Impl;
		Impl* pImpl;
	};
}
