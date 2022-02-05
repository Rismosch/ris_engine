#include "pch.h"

#include <filesystem>

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
		
		auto response	 = locate_asset_folder(allocator_);
		if (response.error != risCompilerError::OK)
		{
			allocator_->free_to_marker(marker);
			return response.error;
		}

		path_to_platform(response.path);
		const auto root_path = response.path.get_buffer();

		if(!std::filesystem::is_directory(root_path))
		{
			allocator_->free_to_marker(marker);
			return risCompilerError::ASSET_FOLDER_MISSING;
		}

		for(const auto& entry : std::filesystem::recursive_directory_iterator(root_path))
		{
			auto entry_path = entry.path();
			const std::ifstream asset_file(entry_path);
			if (!asset_file.good())
				continue;

		}

		allocator_->free_to_marker(marker);
		return risCompilerError::OK;
	}

	risCompilerError risCompiler::decompile_asset_folder()
	{
		return risCompilerError::OK;
	}
}
