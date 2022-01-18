#pragma once
#include "../risData/risData.h"

namespace risMemory
{
	using namespace risData;

	inline U16 swapU16(U16 value)
	{
		return
			static_cast<U16>((value & 0x00FF) << 8) |
			static_cast<U16>((value & 0xFF00) << 8);
	}

	inline U32 swapU32(U32 value)
	{
		return
			static_cast<U32>((value & 0x000000FF) << 24) |
			static_cast<U32>((value & 0x0000FF00) << 8) |
			static_cast<U32>((value & 0x00FF0000) >> 8) |
			static_cast<U32>((value & 0xFF000000) >> 24);
	}

	union U32F32
	{
		U32 m_asU32;
		F32 m_asF32;
	};

	inline F32 swapF32(F32 value)
	{
		U32F32 u{};
		u.m_asF32 = value;
		u.m_asU32 = swapU32(u.m_asU32);

		return u.m_asF32;
	}

	inline U32 convertF32(F32 value)
	{
		U32F32 u{};
		u.m_asF32 = value;

		return u.m_asU32;
	}

	inline F32 convertU32(U32 value)
	{
		U32F32 u{};
		u.m_asU32 = value;

		return u.m_asF32;
	}
}