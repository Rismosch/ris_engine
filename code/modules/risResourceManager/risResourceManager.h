#pragma once

#include "../risData/risString.h"
#include "../risData/risAllocators.h"

namespace risResource
{
	using namespace risData;

	class risResourceManager
	{
	public:
		risResourceManager(const risStackAllocator& file_allocator, const risStackAllocator& resource_allocator, bool should_use_package = false);

		template<class Resource>
		Resource* load(StringId path_id);

	private:
		risStackAllocator file_allocator_;
		risStackAllocator resource_allocator_;

#if defined _DEBUG
		bool should_use_package_;
#endif

	};
}
