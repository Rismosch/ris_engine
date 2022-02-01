#pragma once

#include "risFile.h"
#include "risPath.h"
#include "risResourceError.h"

namespace risEngine
{
	const auto resource_redirect_path = "resource.redirect";

	template<class Allocator>
	risResourceError locate_asset_folder(Allocator* allocator, risPath* path_buffer)
	{
		if (!file_exists(resource_redirect_path))
			return risResourceError::REDIRECT_MISSING;

		std::ifstream read_file;
		read_file.open(resource_redirect_path);

		read_file.seekg(0, std::ios_base::end);
		const auto length = static_cast<U32>(read_file.tellg());
		if (length > MAX_PATH_LENGTH)
		{
			read_file.close();
			return risResourceError::REDIRECT_PATH_TOO_LONG;
		}

		read_file.seekg(0, std::ios_base::beg);

		path_buffer = allocator->alloc_class<risPath>();
		path_buffer->init(allocator, length + 1);

		read_file.read(path_buffer->get_buffer(), length);
		path_buffer->seekp(length, StreamLocation::Beginning);
		read_file.close();

		return risResourceError::OK;
	}
}
