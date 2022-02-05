#include "pch.h"

#include "risCompiler.h"
#include "risCompilerUtility.h"

namespace risEditor
{
	risCompiler::risCompiler(risDoubleStackAllocator* double_stack_allocator) : allocator_(double_stack_allocator){}

	risCompilerError risCompiler::compile_asset_folder()
	{
		if (!allocator_->buffer_is_front())
			allocator_->swap_buffers();

		const auto marker = allocator_->get_marker();
		
		const auto response	 = locate_asset_folder(allocator_);
		if (response.error != risCompilerError::OK)
		{
			allocator_->free_to_marker(marker);
			return response.error;
		}

		path_to_platform(response.path);

		if(!directory_exists(response.path))
		{
			allocator_->free_to_marker(marker);
			return risCompilerError::ASSET_FOLDER_MISSING;
		}

		allocator_->free_to_marker(marker);
		return risCompilerError::OK;
	}

	risCompilerError risCompiler::decompile_asset_folder()
	{
		return risCompilerError::OK;
	}
}
