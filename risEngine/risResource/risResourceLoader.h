#pragma once

#include "../risData/risString.h"
#include "../risData/risAllocators.h"

namespace risEngine
{
	class risResourceLoader
	{
	public:
		risResourceLoader(risDoubleStackAllocator* double_stack_allocator, bool should_use_package = false);

		template<class Resource>
		Resource* load(StringId path_id);

	private:
		risDoubleStackAllocator* double_stack_allocator_;

#if defined _DEBUG
		bool should_use_package_;

		template<class Resource>
		Resource* load_from_file(StringId path_id);
#endif

		template<class Resource>
		Resource* load_from_package(StringId path_id);
	};
}
