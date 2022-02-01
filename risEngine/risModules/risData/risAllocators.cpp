#include "pch.h"
#include "risAllocators.h"

namespace risEngine
{
#pragma region risStackAllocator
	risStackAllocator::risStackAllocator(U32 size_bytes) : data_(new U8[size_bytes]), size_bytes_(size_bytes) { }
	risStackAllocator::~risStackAllocator() { delete[] data_; }

	// allocator policy
	void* risStackAllocator::alloc(U32 size_bytes)
	{
		if (marker_ + size_bytes > size_bytes_)
			return nullptr;

		const auto result = &data_[marker_];
		marker_ += size_bytes;

		return result;
	}

	Marker risStackAllocator::get_marker() const
	{
		return marker_;
	}

	void risStackAllocator::free_to_marker(Marker marker)
	{
		if (marker_ > marker)
			marker_ = marker;
	}

	void risStackAllocator::clear()
	{
		marker_ = static_cast<Marker>(0);
	}
#pragma endregion

#pragma region risDoubleStackAllocator
	risDoubleStackAllocator::risDoubleStackAllocator(U32 size_bytes) : data_(new U8[size_bytes]), size_bytes_(size_bytes), marker_back_(size_bytes){}
	risDoubleStackAllocator::~risDoubleStackAllocator() { delete[] data_; }

	// allocator policy
	void* risDoubleStackAllocator::alloc(U32 size_bytes)
	{
		if (buffer_is_front_)
			return alloc_front(size_bytes);
		else
			return alloc_back(size_bytes);
	}

	Marker risDoubleStackAllocator::get_marker() const
	{
		if (buffer_is_front_)
			return marker_front_;
		else
			return marker_back_;
	}

	void risDoubleStackAllocator::free_to_marker(Marker marker)
	{
		if (buffer_is_front_)
			free_to_marker_front(marker);
		else
			free_to_marker_back(marker);
	}

	void risDoubleStackAllocator::clear()
	{
		if (buffer_is_front_)
			clear_front();
		else
			clear_back();
	}


	// utility
	void risDoubleStackAllocator::swap_buffers()
	{
		buffer_is_front_ = !buffer_is_front_;
	}

	bool risDoubleStackAllocator::buffer_is_front()
	{
		return  buffer_is_front_;
	}

	// specific
	void* risDoubleStackAllocator::alloc_front(U32 size_bytes)
	{
		if (marker_front_ + size_bytes > marker_back_)
			return nullptr;

		const auto result = &data_[marker_front_];
		marker_front_ += size_bytes;

		return result;
	}

	Marker risDoubleStackAllocator::get_marker_front() const
	{
		return marker_front_;
	}

	void risDoubleStackAllocator::free_to_marker_front(Marker marker)
	{
		if (marker_front_ > marker)
			marker_front_ = marker;
	}

	void risDoubleStackAllocator::clear_front()
	{
		marker_front_ = 0;
	}

	void* risDoubleStackAllocator::alloc_back(U32 size_bytes)
	{
		if (marker_back_ - size_bytes < marker_front_)
			return nullptr;

		marker_back_ -= size_bytes;
		const auto result = &data_[marker_back_];

		return result;
	}

	Marker risDoubleStackAllocator::get_marker_back() const
	{
		return marker_back_;
	}

	void risDoubleStackAllocator::free_to_marker_back(Marker marker)
	{
		if (marker <= size_bytes_ && marker_back_ < marker)
			marker_back_ = marker;
	}

	void risDoubleStackAllocator::clear_back()
	{
		marker_back_ = size_bytes_;
	}
#pragma endregion

}
