#include "pch.h"

#include "risResourceManager.h"

namespace risResource
{
	risResourceManager::risResourceManager(const risStackAllocator& file_allocator, const risStackAllocator& resource_allocator, bool should_use_package) :
		file_allocator_(file_allocator),
		resource_allocator_(resource_allocator)
	{
#if defined _DEBUG
		should_use_package_ = should_use_package;  // NOLINT(cppcoreguidelines-prefer-member-initializer)
#endif
	}

	template <class Resource>
	Resource risResourceManager::load(StringId path_id)
	{
	}
}
