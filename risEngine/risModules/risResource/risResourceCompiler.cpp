#include "pch.h"

#include "risResourceCompiler.h"
#include "risFile.h"
#include "risPath.h"
#include "risResourceUtility.h"

namespace risEngine
{
	risResourceCompiler::risResourceCompiler(risDoubleStackAllocator* double_stack_allocator) : allocator_(double_stack_allocator){}

	risResourceError risResourceCompiler::compile()
	{
		if (!allocator_->buffer_is_front())
			allocator_->swap_buffers();

		const auto marker = allocator_->get_marker();
		
		const auto response	 = locate_asset_folder(allocator_);
		if (response.error != risResourceError::OK)
		{
			allocator_->free_to_marker(marker);
			return response.error;
		}

		allocator_->free_to_marker(marker);
		return risResourceError::OK;
	}

	risResourceError risResourceCompiler::decompile()
	{
		return risResourceError::OK;
	}
}
