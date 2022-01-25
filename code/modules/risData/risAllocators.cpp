#include "pch.h"
#include "risAllocators.h"

namespace risData
{
#pragma region risStackAllocator
	risStackAllocator::risStackAllocator(U32 size_bytes) : data_(new U8[size_bytes]), size_bytes_(size_bytes) { }
	risStackAllocator::~risStackAllocator() { delete[] data_; }

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
	risDoubleStackAllocator::risDoubleStackAllocator(U32 size_bytes) : data_(new U8[size_bytes]), size_bytes_(size_bytes), marker_end_(size_bytes){}
	risDoubleStackAllocator::~risDoubleStackAllocator() { delete[] data_; }

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
