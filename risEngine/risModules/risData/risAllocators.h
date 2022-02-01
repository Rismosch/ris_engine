#pragma once
#include "risPrimitives.h"

namespace risEngine
{
	typedef U32 Marker;

	class risStackAllocator
	{
	public:

		explicit risStackAllocator(U32 size_bytes);
		~risStackAllocator();

		// allocator policy
		template<class T> T* alloc_class() { return static_cast<T*>(alloc(sizeof(T))); }
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
		explicit risDoubleStackAllocator(U32 size_bytes);
		~risDoubleStackAllocator();

		// allocator policy
		template<class T> T* alloc_class() { return static_cast<T*>(alloc(sizeof(T))); }
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
