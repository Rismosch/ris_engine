#pragma once
#include "risData.h"

namespace risData
{
	class risAllocator
	{
	public:
		typedef U32 Marker;

		explicit risAllocator(U32 size_bytes);
		~risAllocator();

		void* alloc(U32 size_bytes) const;
		Marker get_marker() const;
		void free_to_marker(Marker marker) const;
		void clear() const;

	private:
		struct Impl;
		Impl* pImpl;
	};
}
