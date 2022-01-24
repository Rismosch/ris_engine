#include "pch.h"
#include "risAllocators.h"

namespace risData
{
	struct risStackAllocator::Impl
	{
		U8* data;
		Marker marker = 0;

		explicit Impl(U32 size_bytes) : data(new U8[size_bytes]) { }
		~Impl() { delete[] data; }
	};

	risStackAllocator::risStackAllocator(U32 size_bytes) : pImpl(new Impl(size_bytes)) { }
	risStackAllocator::~risStackAllocator() { delete pImpl; }

	void* risStackAllocator::alloc(U32 size_bytes) const
	{
		const auto result = &pImpl->data[pImpl->marker];
		pImpl->marker += size_bytes;

		return result;
	}

	risStackAllocator::Marker risStackAllocator::get_marker() const
	{
		return pImpl->marker;
	}

	void risStackAllocator::free_to_marker(Marker marker) const
	{
		if (pImpl->marker > marker)
			pImpl->marker = marker;
	}

	void risStackAllocator::clear() const
	{
		pImpl->marker = static_cast<Marker>(0);
	}
}
