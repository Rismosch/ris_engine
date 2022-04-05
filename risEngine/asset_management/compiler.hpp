#pragma once

#include <risEngine/data/double_stack_allocator.hpp>

namespace risEngine
{

	constexpr I32 MAX_PATH_SIZE = 4096;
	const auto ASSET_REDIRECT_PATH = L"asset.ris_redirect";
	const auto ASSET_LOOKUP_PATH = L"asset.ris_lookup";
	const auto ASSET_PACKAGE_PATH = L"asset.ris_package";

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
