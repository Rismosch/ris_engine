#include "pch.h"
#include "risAllocator.h"

namespace risData
{
	struct risAllocator::Impl
	{
		U8* data;
		Marker marker = 0;

		explicit Impl(U32 size_bytes) : data(new U8[size_bytes]) { }
		~Impl() { delete[] data; }
	};

	risAllocator::risAllocator(U32 size_bytes) : pImpl(new Impl(size_bytes)) { }
	risAllocator::~risAllocator() { delete pImpl; }

	void* risAllocator::alloc(U32 size_bytes) const
	{
		const auto result = &pImpl->data[pImpl->marker];
		pImpl->marker += size_bytes;

		return result;
	}

	risAllocator::Marker risAllocator::get_marker() const
	{
		return pImpl->marker;
	}

	void risAllocator::free_to_marker(Marker marker) const
	{
		if (pImpl->marker > marker)
			pImpl->marker = marker;
	}

	void risAllocator::clear() const
	{
		pImpl->marker = static_cast<Marker>(0);
	}
}
