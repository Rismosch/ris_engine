#pragma once
#include "risAllocators.h"

namespace risEngine
{
	struct risDoubleStackAllocatorJanitor
	{
		risDoubleStackAllocator& allocator;
		Marker marker_front;
		Marker marker_back;

		risDoubleStackAllocatorJanitor(const risDoubleStackAllocatorJanitor& other) = default;

		risDoubleStackAllocatorJanitor(risDoubleStackAllocatorJanitor&& other) noexcept
			: allocator(other.allocator),
			marker_front(other.marker_front),
			marker_back(other.marker_back) {}

		risDoubleStackAllocatorJanitor& operator=(const risDoubleStackAllocatorJanitor& other)
		{
			if (this == &other)
				return *this;
			allocator = other.allocator;
			marker_front = other.marker_front;
			marker_back = other.marker_back;
			return *this;
		}

		risDoubleStackAllocatorJanitor& operator=(risDoubleStackAllocatorJanitor&& other) noexcept
		{
			if (this == &other)
				return *this;
			allocator = other.allocator;
			marker_front = other.marker_front;
			marker_back = other.marker_back;
			return *this;
		}

		risDoubleStackAllocatorJanitor(risDoubleStackAllocator& allocator)
			:allocator(allocator), marker_front(allocator.get_marker_front()), marker_back(allocator.get_marker_back())
		{
			if (!allocator.buffer_is_front())
				allocator.swap_buffers();
		}

		~risDoubleStackAllocatorJanitor()
		{
			allocator.free_to_marker_front(marker_front);
			allocator.free_to_marker_back(marker_back);
		}
	};
}
