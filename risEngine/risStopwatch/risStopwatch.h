#pragma once

#include <chrono>

#include "../risData/risPrimitives.h"

namespace risEngine
{
	constexpr U32 risDefaultTimeunit = 600;
	constexpr U32 risMilliSecond = 1000;
	constexpr F32 risSecond = static_cast<F32>(1);
	constexpr F32 risMinute = static_cast<F32>(1) / static_cast<F32>(60);
	constexpr F32 risHour = static_cast<F32>(1) / static_cast<F32>(3600);
	constexpr F32 risDay = static_cast<F32>(1) / static_cast<F32>(86400);

	class risStopwatch
	{
		typedef std::chrono::high_resolution_clock risClock;

	public:
		risStopwatch();

		void reset();

		U64 elapsed_cycles() const;
		U64 reset_cycles();

		template <typename T = U32>
		T elapsed(T unit = risDefaultTimeunit) const;

		template <typename T = U32>
		T reset(T unit = risDefaultTimeunit);

	private:
		risClock::time_point last_;

		template<typename T>
		static T cycles_to_unit(U64 duration, T unit);
	};
}
