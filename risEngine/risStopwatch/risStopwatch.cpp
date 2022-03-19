#include "pch.h"

#include "risStopwatch.h"

namespace risEngine
{
	risStopwatch::risStopwatch() : last_(risClock::now()) {}

	void risStopwatch::reset()
	{
		*this = risStopwatch();
	}

	U64 risStopwatch::elapsed_cycles() const
	{
		return (risClock::now() - last_).count();
	}

	U64 risStopwatch::reset_cycles()
	{
		const auto now = risClock::now();
		const auto elapsed = now - last_;
		last_ = now;
		return elapsed.count();
	}

	template <typename T>
	T risStopwatch::elapsed(T unit) const
	{
		const auto duration = elapsed_cycles();
		return cycles_to_unit<T>(duration, unit);
	}

	template <typename T>
	T risStopwatch::reset(T unit)
	{
		const auto duration = reset_cycles();
		return cycles_to_unit<T>(duration, unit);
	}

	template<typename T>
	static T risStopwatch::cycles_to_unit(U64 duration, T unit)
	{
		return unit * duration * static_cast<T>(risClock::period::num) / static_cast<T>(risClock::period::den);
	}

	template U32 risStopwatch::elapsed(U32 unit) const;
	template U32 risStopwatch::reset(U32 unit);
	template U32 risStopwatch::cycles_to_unit(U64 duration, U32 unit);

	template F32 risStopwatch::elapsed(F32 unit) const;
	template F32 risStopwatch::reset(F32 unit);
	template F32 risStopwatch::cycles_to_unit(U64 duration, F32 unit);
}