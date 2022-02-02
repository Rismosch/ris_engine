#pragma once

#include "risFile.h"
#include "risPath.h"
#include "risResourceError.h"

namespace risEngine
{
	const auto resource_redirect_path = "resource.redirect";

	struct AssetFolderResponse
	{
		risResourceError error;
		risPath path;

		static AssetFolderResponse Error(risResourceError input_error)
		{
			AssetFolderResponse response{};
			response.error = input_error;

			return response;
		}

		static AssetFolderResponse Success(risPath input_path)
		{
			AssetFolderResponse response{};
			response.error = risResourceError::OK;
			response.path = input_path;

			return response;
		}
	};

	template<class Allocator>
	AssetFolderResponse locate_asset_folder(Allocator* allocator)
	{

		if (!file_exists(resource_redirect_path))
			return AssetFolderResponse::Error(risResourceError::REDIRECT_MISSING);

		std::ifstream read_file;
		read_file.open(resource_redirect_path);

		read_file.seekg(0, std::ios_base::end);
		const auto length = static_cast<U32>(read_file.tellg());
		if (length > MAX_PATH_LENGTH)
		{
			read_file.close();
			return AssetFolderResponse::Error(risResourceError::REDIRECT_PATH_TOO_LONG);
		}

		read_file.seekg(0, std::ios_base::beg);

		auto path = risPath(allocator, length + 1);

		read_file.read(path.get_buffer(), length);
		path.seekp(length, StreamLocation::Beginning);
		read_file.close();

		return AssetFolderResponse::Success(path);
	}
}
