#include "pch.h"

#include "risResourceCompiler.h"
#include "risFile.h"

namespace risEngine
{
	risResourceCompiler::risResourceCompiler(risDoubleStackAllocator* double_stack_allocator) :
		double_stack_allocator_(double_stack_allocator){}

	risCompilerError risResourceCompiler::compile()
	{
		const auto resource_redirect_path = "resource.redirect";
		if (!file_exists(resource_redirect_path))
			return risCompilerError::REDIRECT_MISSING;

		std::ifstream read_file;
		read_file.open(resource_redirect_path);

		read_file.seekg(0, std::ios_base::end);
		auto length = read_file.tellg();
		read_file.seekg(0, std::ios_base::beg);

		read_file.close();

		return risCompilerError::OK;
	}

	risCompilerError risResourceCompiler::decompile()
	{
		return risCompilerError::OK;
	}
}
