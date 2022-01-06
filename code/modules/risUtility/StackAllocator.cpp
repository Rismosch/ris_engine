#include "pch.h"
#include "StackAllocator.h"

namespace risUtility
{
	struct StackAllocator::Impl
	{
		U8* data;
		Marker marker = 0;

		explicit Impl(U32 size_bytes) : data(new U8[size_bytes]) { }
		~Impl() { delete[] data; }
	};

	StackAllocator::StackAllocator(U32 size_bytes) : pImpl(new Impl(size_bytes)) { }
	StackAllocator::~StackAllocator() { delete pImpl; }

	void* StackAllocator::alloc(U32 size_bytes) const
	{
		const auto result = &pImpl->data[pImpl->marker];
		pImpl->marker += size_bytes;

		return result;
	}

	StackAllocator::Marker StackAllocator::get_marker() const
	{
		return pImpl->marker;
	}

	void StackAllocator::free_to_marker(Marker marker) const
	{
		if (pImpl->marker > marker)
			pImpl->marker = marker;
	}

	void StackAllocator::clear() const
	{
		pImpl->marker = static_cast<Marker>(0);
	}
}
