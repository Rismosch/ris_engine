#include "pch.h"

#include <risEngine/data/double_stack_allocator.hpp>

using namespace risEngine;

class risStackAllocatorTests : public ::testing::Test
{
protected:
	risDoubleStackAllocator allocator;

	void SetUp() override
	{
		allocator = risDoubleStackAllocator();
		allocator.init(10);
	}

	void TearDown() override
	{
		allocator.release();
	}
};

TEST_F(risStackAllocatorTests, ShouldAllocateFrontAndBack)
{
	const auto pointer1 = allocator.alloc_front(1);
	const auto pointer2 = allocator.alloc_back(1);

	EXPECT_NE(pointer1, nullptr);
	EXPECT_NE(pointer2, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldNotAllocateWhenSizeDoesNotFit)
{
	const auto pointer1 = allocator.alloc_front(100);
	const auto pointer2 = allocator.alloc_back(100);

	allocator.alloc_front(1);
	allocator.alloc_back(1);

	const auto pointer3 = allocator.alloc_front(9);
	const auto pointer4 = allocator.alloc_back(9);

	allocator.alloc_front(8);

	const auto pointer5 = allocator.alloc_front(1);
	const auto pointer6 = allocator.alloc_back(1);

	EXPECT_EQ(pointer1, nullptr);
	EXPECT_EQ(pointer2, nullptr);
	EXPECT_EQ(pointer3, nullptr);
	EXPECT_EQ(pointer4, nullptr);
	EXPECT_EQ(pointer5, nullptr);
	EXPECT_EQ(pointer6, nullptr);
}

TEST_F(risStackAllocatorTests, ShouldAllocateAndFreeFront)
{
	const auto marker1 = allocator.get_marker_front();
	allocator.alloc_front(1);
	const auto marker2 = allocator.get_marker_front();
	allocator.free_to_marker_front(marker1);
	const auto marker3 = allocator.get_marker_front();

	EXPECT_LT(marker1, marker2);
	EXPECT_EQ(marker1, marker3);
	EXPECT_GT(marker2, marker3);
}

TEST_F(risStackAllocatorTests, ShouldAllocateAndFreeBack)
{
	const auto marker1 = allocator.get_marker_back();
	allocator.alloc_back(1);
	const auto marker2 = allocator.get_marker_back();
	allocator.free_to_marker_back(marker1);
	const auto marker3 = allocator.get_marker_back();

	EXPECT_GT(marker1, marker2);
	EXPECT_EQ(marker1, marker3);
	EXPECT_LT(marker2, marker3);
}

TEST_F(risStackAllocatorTests, ShouldClearFront)
{
	const auto marker1 = allocator.get_marker_front();
	allocator.alloc_front(1);
	allocator.alloc_front(1);
	allocator.alloc_front(1);
	allocator.alloc_front(1);
	const auto marker2 = allocator.get_marker_front();
	allocator.clear_front();
	const auto marker3 = allocator.get_marker_front();

	EXPECT_LT(marker1, marker2);
	EXPECT_EQ(marker1, marker3);
	EXPECT_GT(marker2, marker3);
}

TEST_F(risStackAllocatorTests, ShouldClearBack)
{
	const auto marker1 = allocator.get_marker_back();
	allocator.alloc_back(1);
	allocator.alloc_back(1);
	allocator.alloc_back(1);
	allocator.alloc_back(1);
	const auto marker2 = allocator.get_marker_back();
	allocator.clear_back();
	const auto marker3 = allocator.get_marker_back();

	EXPECT_GT(marker1, marker2);
	EXPECT_EQ(marker1, marker3);
	EXPECT_LT(marker2, marker3);
}

TEST_F(risStackAllocatorTests, ShouldUseFrontOnInitialize)
{
	EXPECT_TRUE(allocator.buffer_is_front());

	const auto marker1 = allocator.get_marker();
	allocator.alloc(1);
	allocator.alloc(1);
	const auto marker2 = allocator.get_marker();
	allocator.alloc(1);
	allocator.alloc(1);
	const auto marker3 = allocator.get_marker();
	allocator.free_to_marker(marker2);
	const auto marker4 = allocator.get_marker();
	allocator.clear();
	const auto marker5 = allocator.get_marker();

	EXPECT_LT(marker1, marker2);
	EXPECT_LT(marker2, marker3);
	EXPECT_GT(marker3, marker4);
	EXPECT_GT(marker4, marker5);
	EXPECT_EQ(marker1, marker5);
	EXPECT_EQ(marker2, marker4);
}

TEST_F(risStackAllocatorTests, ShouldUseFrontOnSwap)
{
	allocator.swap_buffers();
	EXPECT_FALSE(allocator.buffer_is_front());

	const auto marker1 = allocator.get_marker();
	allocator.alloc(1);
	allocator.alloc(1);
	const auto marker2 = allocator.get_marker();
	allocator.alloc(1);
	allocator.alloc(1);
	const auto marker3 = allocator.get_marker();
	allocator.free_to_marker(marker2);
	const auto marker4 = allocator.get_marker();
	allocator.clear();
	const auto marker5 = allocator.get_marker();

	EXPECT_GT(marker1, marker2);
	EXPECT_GT(marker2, marker3);
	EXPECT_LT(marker3, marker4);
	EXPECT_LT(marker4, marker5);
	EXPECT_EQ(marker1, marker5);
	EXPECT_EQ(marker2, marker4);
}