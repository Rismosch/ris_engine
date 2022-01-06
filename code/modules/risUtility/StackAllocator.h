#pragma once
#include "../risData/risData.h"

namespace risUtility
{
	using namespace risData;

	class StackAllocator
	{
	public:
		typedef U32 Marker;

		explicit StackAllocator(U32 size_bytes);
		~StackAllocator();

		void* alloc(U32 size_bytes) const;
		Marker get_marker() const;
		void free_to_marker(Marker marker) const;
		void clear() const;

	private:
		struct Impl;
		Impl* pImpl;
	};
}
