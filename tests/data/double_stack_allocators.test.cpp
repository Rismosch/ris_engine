#include "pch.h"

#include <risEngine/data/double_stack_allocator.hpp>

using namespace risEngine;

class risStackAllocatorTests : public ::testing::Test
{
protected:
	risDoubleStackAllocator allocator;

	Marker marker;

	I32* number1 = nullptr;
	I32* number2 = nullptr;
	I32* number3 = nullptr;

	const I32 expected1 = 42;
	const I32 expected2 = 13;
	const I32 expected3 = -17;
	const I32 expected4 = 0;
	const I32 expected5 = 100;
	const I32 expected6 = 5040;

	void SetUp() override
	{
		allocator = risDoubleStackAllocator();
		allocator.init(sizeof(I32) * 3);

		number1 = static_cast<I32*>(allocator.alloc(sizeof(I32)));
		marker = allocator.get_marker();
		number2 = static_cast<I32*>(allocator.alloc(sizeof(I32)));
		number3 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

		*number1 = expected1;
		*number2 = expected2;
		*number3 = expected3;
	}

	void TearDown() override
	{
		allocator.release();
	}
};

TEST_F(risStackAllocatorTests, ShouldAllocate)
{
	EXPECT_EQ(*(number1 + 0), expected1);
	EXPECT_EQ(*(number1 + 1), expected2);
	EXPECT_EQ(*(number1 + 2), expected3);
}

TEST_F(risStackAllocatorTests, ShouldClear)
{
	allocator.clear();

	const auto number4 = static_cast<I32*>(allocator.alloc(sizeof(I32)));
	const auto number5 = static_cast<I32*>(allocator.alloc(sizeof(I32)));
	const auto number6 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

	*number4 = expected4;
	*number5 = expected5;
	*number6 = expected6;

	EXPECT_EQ(*(number1 + 0), expected4);
	EXPECT_EQ(*(number1 + 1), expected5);
	EXPECT_EQ(*(number1 + 2), expected6);
}

TEST_F(risStackAllocatorTests, ShouldFreeToMarker)
{
	allocator.free_to_marker(marker);

	const auto number4 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

	*number4 = expected4;

	EXPECT_EQ(*(number1 + 0), expected1);
	EXPECT_EQ(*(number1 + 1), expected4);
	EXPECT_EQ(*(number1 + 2), expected3);
}

TEST_F(risStackAllocatorTests, ShouldNotAllocateWhenFull)
{
	const auto number4 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

	EXPECT_EQ(number4, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldNotAllocateWhenTooBig)
{
	allocator.clear();
	const auto too_big = allocator.alloc(sizeof(I32) * 4);

	EXPECT_EQ(too_big, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldNotFreeToBiggerMarker)
{
	FAIL() << "add more tests for backbuffer";
}