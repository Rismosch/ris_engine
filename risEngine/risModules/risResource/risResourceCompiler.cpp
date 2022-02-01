#include "pch.h"

#include "risResourceCompiler.h"
#include "risFile.h"

namespace risEngine
{
	risResourceCompiler::risResourceCompiler(risDoubleStackAllocator* double_stack_allocator) :
		double_stack_allocator_(double_stack_allocator){}

	risCompilerError risResourceCompiler::compile()
	{
		const auto resource_redirect = "resource.redirect";
		if (!file_exists(resource_redirect))
			return risCompilerError::REDIRECT_MISSING;

		return risCompilerError::OK;
	}

	risCompilerError risResourceCompiler::decompile()
	{
		return risCompilerError::OK;
	}
}
