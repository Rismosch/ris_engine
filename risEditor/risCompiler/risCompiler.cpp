#include "pch.h"

#include <filesystem>
#include <string>

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

			auto asset_path = entry_path.c_str();
			I32 path_length = 0;
			I32 dot_position = 0;
			while (asset_path[path_length] != 0)
			{
				if (asset_path[path_length] == '.')
					dot_position = path_length;

				++path_length;
			}

			auto extension_marker = allocator_->get_marker();

			auto extension_length = path_length - dot_position;
			auto extension = static_cast<wchar_t*>(allocator_->alloc(sizeof(wchar_t) * extension_length));

			I32 i = 0;
			for (; i + 1 < extension_length; ++i)
			{
				extension[i] = asset_path[dot_position + 1 + i];
			}
			extension[i] = 0;

			allocator_->free_to_marker(extension_marker);
		}

		allocator_->free_to_marker(marker);
		return risCompilerError::OK;
	}

	risCompilerError risCompiler::decompile_asset_folder()
	{
		return risCompilerError::OK;
	}
}
