#pragma once

#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	template <I32 v>
	struct Int2Type
	{
		enum {value = v};
	};
}
