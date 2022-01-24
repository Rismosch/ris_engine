#pragma once
#include "risPrimitives.h"

namespace risData
{
	class risStackAllocator
	{
	public:
		typedef U32 Marker;

		explicit risStackAllocator(U32 size_bytes);
		~risStackAllocator();

		void* alloc(U32 size_bytes) const;
		Marker get_marker() const;
		void free_to_marker(Marker marker) const;
		void clear() const;

	private:
		struct Impl;
		Impl* pImpl;
	};
}
