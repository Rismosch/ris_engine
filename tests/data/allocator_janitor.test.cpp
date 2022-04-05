#include "pch.h"

#include <risEngine/data/allocator_janitor.hpp>

using namespace risEngine;

TEST(risAllocatorJanitorTests, ShouldCleanUpAfterFallingOutOfScope)
{
	auto allocator = risDoubleStackAllocator();
	allocator.init(10);
	allocator.release();
	allocator.alloc_front(1);
	allocator.alloc_front(2);

	Marker front1;
	Marker front2;
	Marker front3;
	Marker back1;
	Marker back2;
	Marker back3;

	bool is_front_buffer1;
	bool is_front_buffer2;
	bool is_front_buffer3;

	front1 = allocator.get_marker_front();
	back1 = allocator.get_marker_back();
	is_front_buffer1 = allocator.buffer_is_front();

	{
		auto janitor = risAllocatorJanitor(allocator);

		janitor.allocator.alloc_front(3);
		janitor.allocator.alloc_back(4);
		janitor.allocator.swap_buffers();

		front2 = allocator.get_marker_front();
		back2 = allocator.get_marker_front();
		is_front_buffer2 = allocator.buffer_is_front();
	}

	front3 = allocator.get_marker_front();
	back3 = allocator.get_marker_back();
	is_front_buffer3 = allocator.buffer_is_front();

	EXPECT_NE(front1, front2);
	EXPECT_EQ(front1, front3);
	EXPECT_NE(front2, front3);

	EXPECT_NE(back1, back2);
	EXPECT_EQ(back1, back3);
	EXPECT_NE(back2, back3);

	EXPECT_NE(is_front_buffer1, is_front_buffer2);
	EXPECT_EQ(is_front_buffer1, is_front_buffer3);
	EXPECT_NE(is_front_buffer2, is_front_buffer3);
}