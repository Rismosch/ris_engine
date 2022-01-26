#include "pch.h"

#include "risResourceManager.h"

namespace risResource
{
	risResourceManager::risResourceManager(const risDoubleStackAllocator& double_stack_allocator, bool should_use_package) :
		double_stack_allocator_(double_stack_allocator)
	{
#if defined _DEBUG
		should_use_package_ = should_use_package;  // NOLINT(cppcoreguidelines-prefer-member-initializer)
#endif
	}

	void risResourceManager::compile()
	{
		
	}

	void risResourceManager::decompile()
	{

	}

	template <class Resource>
	Resource* risResourceManager::load(StringId path_id)
	{
#if defined _DEBUG
		if (should_use_package_)
			return load_from_package<Resource>(path_id);
		else
			return load_from_file<Resource>(path_id);
#else
		return load_from_package<Resource>(path_id);
#endif
	}

#if defined _DEBUG
	template <class Resource>
	Resource* risResourceManager::load_from_file(StringId path_id)
	{
		auto path = internal_string(path_id);
	}
#endif

	template <class Resource>
	Resource* risResourceManager::load_from_package(StringId path_id)
	{

	}

}
