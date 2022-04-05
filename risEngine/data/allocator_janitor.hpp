#pragma once
#include <risEngine/data/double_stack_allocator.hpp>

namespace risEngine
{
	struct risAllocatorJanitor
	{
		risDoubleStackAllocator& allocator;
		Marker marker_front;
		Marker marker_back;
		bool buffer_is_front;

		risAllocatorJanitor(const risAllocatorJanitor& other) = default;

		risAllocatorJanitor(risAllocatorJanitor&& other) noexcept
			: allocator(other.allocator),
			marker_front(other.marker_front),
			marker_back(other.marker_back) {}

		risAllocatorJanitor& operator=(const risAllocatorJanitor& other)
		{
			if (this == &other)
				return *this;
			allocator = other.allocator;
			marker_front = other.marker_front;
			marker_back = other.marker_back;
			return *this;
		}

		risAllocatorJanitor& operator=(risAllocatorJanitor&& other) noexcept
		{
			if (this == &other)
				return *this;
			allocator = other.allocator;
			marker_front = other.marker_front;
			marker_back = other.marker_back;
			return *this;
		}

		risAllocatorJanitor(risDoubleStackAllocator& allocator):
			allocator(allocator),
			marker_front(allocator.get_marker_front()),
			marker_back(allocator.get_marker_back()),
			buffer_is_front(allocator.buffer_is_front())
		{
			if (!allocator.buffer_is_front())
				allocator.swap_buffers();
		}

		~risAllocatorJanitor()
		{
			allocator.free_to_marker_front(marker_front);
			allocator.free_to_marker_back(marker_back);
			if (allocator.buffer_is_front() != buffer_is_front)
				allocator.swap_buffers();
		}
	};
}
