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

		template<typename T>
		static T cycles_to_unit(U64 duration, T unit)
		{
			return unit * duration * static_cast<T>(risClock::period::num) / static_cast<T>(risClock::period::den);
		}

	public:
		risStopwatch() : last_(risClock::now()) {}

		void reset()
		{
			*this = risStopwatch();
		}

		U64 elapsed_cycles() const
		{
			return (risClock::now() - last_).count();
		}

		U64 reset_cycles()
		{
			const auto now = risClock::now();
			const auto elapsed = now - last_;
			last_ = now;
			return elapsed.count();
		}

		template <typename T = U32>
		T elapsed(T unit = risDefaultTimeunit) const
		{
			const auto duration = elapsed_cycles();
			return cycles_to_unit<T>(duration, unit);
		}

		template <typename T = U32>
		T reset(T unit = risDefaultTimeunit)
		{
			const auto duration = reset_cycles();
			return cycles_to_unit<T>(duration, unit);
		}

	private:
		risClock::time_point last_;
	};
}
