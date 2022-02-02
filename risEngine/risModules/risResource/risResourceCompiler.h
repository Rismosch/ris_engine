#pragma once

#include "risResourceError.h"
#include "../risData/risAllocators.h"

namespace risEngine
{
	class risResourceCompiler
	{
	public:
		risResourceCompiler(risDoubleStackAllocator* double_stack_allocator);

		risResourceError compile();
		risResourceError decompile();

	private:
		risDoubleStackAllocator* allocator_ = nullptr;
	};
}
