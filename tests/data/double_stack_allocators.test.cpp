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

	void setup_base()
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

	void setup_front()
	{
		
	}

	void setup_back()
	{
		
	}

	void setup_both()
	{
		
	}

	void TearDown() override
	{
		allocator.release();
	}
};

TEST_F(risStackAllocatorTests, ShouldAllocate)
{
	setup_base();

	EXPECT_EQ(*(number1 + 0), expected1);
	EXPECT_EQ(*(number1 + 1), expected2);
	EXPECT_EQ(*(number1 + 2), expected3);
}

TEST_F(risStackAllocatorTests, ShouldClear)
{
	setup_base();

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
	setup_base();

	allocator.free_to_marker(marker);

	const auto number4 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

	*number4 = expected4;

	EXPECT_EQ(*(number1 + 0), expected1);
	EXPECT_EQ(*(number1 + 1), expected4);
	EXPECT_EQ(*(number1 + 2), expected3);
}

TEST_F(risStackAllocatorTests, ShouldNotAllocateWhenFull)
{
	setup_base();

	const auto number4 = static_cast<I32*>(allocator.alloc(sizeof(I32)));

	EXPECT_EQ(number4, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldNotAllocateWhenTooBig)
{
	setup_base();

	allocator.clear();
	const auto too_big = allocator.alloc(sizeof(I32) * 4);

	EXPECT_EQ(too_big, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldNotFreeToBiggerMarker)
{
	setup_base();

	Marker marker1 = allocator.get_marker();
	allocator.free_to_marker(marker);
	Marker marker2 = allocator.get_marker();
	allocator.free_to_marker(marker1);
	Marker marker3 = allocator.get_marker();

	EXPECT_NE(marker1, marker2);
	EXPECT_NE(marker1, marker3);
	EXPECT_EQ(marker2, marker3);
}

TEST_F(risStackAllocatorTests, hmm)
{
	FAIL() << "implement tests for front and backbuffer";
}