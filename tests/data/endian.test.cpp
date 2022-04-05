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

	EXPECT_EQ(swapped, 0x78563412);
}

TEST(risEndianTests, ShouldSwapF32)
{
	const U32 original = 0x12345678;
	const F32 converted = convertU32(original);
	const F32 swapped = swapF32(converted);
	const U32 result = convertF32(swapped);
	
	EXPECT_EQ(result, 0x78563412);
}

TEST(risEndianTests, ShouldConvertF32)
{
	const U32 original = 0x3ec00000;
	const F32 converted = convertU32(original);

	EXPECT_EQ(converted, 0.375f);
}

TEST(risEndianTests, ShouldConvertU32)
{
	const F32 original = 123.456f;
	const U32 converted = convertF32(original);

	EXPECT_EQ(converted, 0x42F6E979);
}