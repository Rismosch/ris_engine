#include "pch.h"

#include "risResourceCompiler.h"
#include "risFile.h"
#include "risPath.h"

namespace risEngine
{
	void risResourceCompiler::init(risDoubleStackAllocator* double_stack_allocator)
	{
		double_stack_allocator_ = double_stack_allocator;
	}

	risCompilerError risResourceCompiler::compile()
	{
		const auto resource_redirect_path = "resource.redirect";
		if (!file_exists(resource_redirect_path))
			return risCompilerError::REDIRECT_MISSING;

		if (!double_stack_allocator_->buffer_is_front())
			double_stack_allocator_->swap_buffers();

		const auto marker = double_stack_allocator_->get_marker();

		std::ifstream read_file;
		read_file.open(resource_redirect_path);

		read_file.seekg(0, std::ios_base::end);
		const auto length = static_cast<U32>(read_file.tellg());
		read_file.seekg(0, std::ios_base::beg);

		auto path_buffer = static_cast<risPathBuffer*>(double_stack_allocator_->alloc(sizeof(risPathBuffer)));
		path_buffer->init(double_stack_allocator_, MAX_PATH_LENGTH);

		read_file.read(path_buffer->get_buffer(), MAX_PATH_LENGTH);
		read_file.close();

		double_stack_allocator_->free_to_marker(marker);

		return risCompilerError::OK;
	}

	risCompilerError risResourceCompiler::decompile()
	{
		return risCompilerError::OK;
	}
}
