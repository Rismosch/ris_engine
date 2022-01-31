#pragma once

#include "../risData/risAllocators.h"

namespace risResource
{
	using namespace risData;

	class risResourceCompiler
	{
	public:
		risResourceCompiler(risDoubleStackAllocator* double_stack_allocator);

		void compile();
		void decompile();

	private:
		risDoubleStackAllocator* double_stack_allocator_;
	};
}
