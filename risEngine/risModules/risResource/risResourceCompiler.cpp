#include "pch.h"

#include "risResourceCompiler.h"
#include "risFile.h"
#include "risPath.h"
#include "risResourceUtility.h"

namespace risEngine
{
	void risResourceCompiler::init(risDoubleStackAllocator* double_stack_allocator)
	{
		allocator_ = double_stack_allocator;
	}

	risResourceError risResourceCompiler::compile()
	{
		if (!allocator_->buffer_is_front())
			allocator_->swap_buffers();

		const auto marker = allocator_->get_marker();

		risPath* path_buffer = nullptr;
		const auto error = locate_asset_folder(allocator_, path_buffer);
		if (error != risResourceError::OK)
		{
			allocator_->free_to_marker(marker);
			return error;
		}

		auto test = new char[100];
		path_buffer->get_encoded_string(test, 100);
		delete[] test;

		allocator_->free_to_marker(marker);
		return risResourceError::OK;
	}

	risResourceError risResourceCompiler::decompile()
	{
		return risResourceError::OK;
	}
}
