#pragma once

namespace risEngine
{
	enum class risResourceError
	{
		// build was successful, no issues found
		OK,

		// asset redirect file was not found
		REDIRECT_MISSING,

		// redirect path is too long
		REDIRECT_PATH_TOO_LONG,

		// found asset path does not exist or could not be opened
		ASSET_FOLDER_MISSING,
	};
}
