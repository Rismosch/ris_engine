#pragma once

#include "../risData/risString.h"
#include "../risData/risAllocators.h"

namespace risResource
{
	using namespace risData;

	class risResourceManager
	{
	public:
		risResourceManager(risStackAllocator stack_allocator, bool should_use_package = false);

		template<class Resource>
		Resource load(StringId path_id);
	private:
		risStackAllocator stack_allocator_;
		bool should_use_package_;
	};
}
