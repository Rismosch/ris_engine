#pragma once

#include "../../risEngine/risData/risAllocators.h"

namespace risCompiler
{
	using namespace risEngine;

	constexpr I32 MAX_PATH_LENGTH = 4096;
	const auto RESOURCE_REDIRECT_PATH = "resource.redirect";

	enum class risCompilerError
	{
		// build was successful, no issues found
		OK,

		// asset redirect file was not found
		// make sure that a file called "resource.redirect" exists in the same directory as the execution directory
		REDIRECT_MISSING,

		// redirect path is too long
		REDIRECT_PATH_TOO_LONG,

		// found asset path does not exist or could not be opened
		ASSET_FOLDER_MISSING,
	};

	extern risCompilerError compile_assets(risDoubleStackAllocator& allocator);
	// extern risCompilerError decompile_resources();
}
