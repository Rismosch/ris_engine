#pragma once
#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	typedef I32 Marker;

	struct risDoubleStackAllocator
	{
		void init(I32 size_bytes);
		void release() const;

		// allocator policy
		void* alloc(I32 size_bytes);
		Marker get_marker() const;
		void free_to_marker(Marker marker);
		void clear();

		// utility
		void swap_buffers();
		bool buffer_is_front() const;

		// specific
		void* alloc_front(I32 size_bytes);
		Marker get_marker_front() const;
		void free_to_marker_front(Marker marker);
		void clear_front();
		
		void* alloc_back(I32 size_bytes);
		Marker get_marker_back() const;
		void free_to_marker_back(Marker marker);
		void clear_back();

	private:
		U8* data_ = nullptr;
		I32 size_bytes_ = 0;

		Marker marker_front_ = 0;
		Marker marker_back_ = 0;

		bool buffer_is_front_ = true;
	};
}
