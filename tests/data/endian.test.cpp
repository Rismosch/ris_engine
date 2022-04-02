#include "pch.h"

#include <risEngine/data/primitives.hpp>
#include <risEngine/data/endian.hpp>

using namespace risEngine;

TEST(risEndianTests, ShouldSwapU16)
{
	const U16 original = 0x1234;
	const U16 swapped = swapU16(original);

	EXPECT_EQ(swapped, 0x3412);
}

TEST(risEndianTests, ShouldSwapU32)
{
	const U32 original = 0x12345678;
	const U32 swapped = swapU32(original);

	EXPECT_EQ(swapped, 0x078563412);
}

TEST(risEndianTests, ShouldSwapF32)
{
	const F32 original = 42.f;
	const F32 swapped = swapU32(original);
	
	FAIL();
}

TEST(risEndianTests, ShouldConvertF32)
{
	const U32 original = 0x12345678;
	const U32 swapped = swapU32(original);

	FAIL();
}

TEST(risEndianTests, ShouldConvertU32)
{
	const U32 original = 0x12345678;
	const U32 swapped = swapU32(original);

	FAIL();
}