#pragma once
#include "risPrimitives.h"

namespace risData
{
	typedef U32 Marker;

	class risStackAllocator
	{
	public:

		explicit risStackAllocator(U32 size_bytes);
		~risStackAllocator();

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
	};
}
