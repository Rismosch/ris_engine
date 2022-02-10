#pragma once
#include "risPrimitives.h"

namespace risEngine
{
	typedef U32 Marker;

	class risStackAllocator
	{
	public:
		// constructors
		explicit risStackAllocator(U32 size_bytes);
		~risStackAllocator();
		risStackAllocator(const risStackAllocator& other) = default;
		risStackAllocator(risStackAllocator&& other) noexcept;
		risStackAllocator& operator=(const risStackAllocator& other);
		risStackAllocator& operator=(risStackAllocator&& other) noexcept;

		// allocator policy
		void* alloc(U32 size_bytes);
		Marker get_marker() const;
		void free_to_marker(Marker marker);
		void clear();

	private:
		U8* data_;
		U32 size_bytes_ = 0;

		Marker marker_ = 0;
	};

	class risDoubleStackAllocator
	{
	public:
		// constructors
		explicit risDoubleStackAllocator(U32 size_bytes);
		~risDoubleStackAllocator();
		risDoubleStackAllocator(const risDoubleStackAllocator& other) = default;
		risDoubleStackAllocator(risDoubleStackAllocator&& other) noexcept;
		risDoubleStackAllocator& operator=(const risDoubleStackAllocator& other);
		risDoubleStackAllocator& operator=(risDoubleStackAllocator&& other) noexcept;

		// allocator policy
		void* alloc(U32 size_bytes);
		Marker get_marker() const;
		void free_to_marker(Marker marker);
		void clear();

		// utility
		void swap_buffers();
		bool buffer_is_front() const;

		// specific
		void* alloc_front(U32 size_bytes);
		Marker get_marker_front() const;
		void free_to_marker_front(Marker marker);
		void clear_front();
		
		void* alloc_back(U32 size_bytes);
		Marker get_marker_back() const;
		void free_to_marker_back(Marker marker);
		void clear_back();

	private:
		U8* data_;
		U32 size_bytes_ = 0;

		Marker marker_front_ = 0;
		Marker marker_back_ = 0;

		bool buffer_is_front_ = true;
	};
}
