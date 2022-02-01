#pragma once

#include "../risData/risAllocators.h"

namespace risEngine
{
	enum class risCompilerError
	{
		OK,
		REDIRECT_MISSING
	};

	class risResourceCompiler
	{
	public:
		risResourceCompiler(risDoubleStackAllocator* double_stack_allocator);

		risCompilerError compile();
		risCompilerError decompile();

	private:
		risDoubleStackAllocator* double_stack_allocator_;
	};
}
