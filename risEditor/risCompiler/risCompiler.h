#pragma once

#include "risCompilerError.h"
#include "../../risEngine/risModules/risData/risAllocators.h"

namespace risEditor
{
	using namespace risEngine;

	class risCompiler
	{
	public:
		risCompiler(risDoubleStackAllocator* double_stack_allocator);

		risCompilerError compile_asset_folder();
		risCompilerError decompile_asset_folder();

	private:
		risDoubleStackAllocator* allocator_ = nullptr;
	};
}
