#pragma once

#include "../risData/risPrimitives.h"
#include "../risData/risString.h"

namespace risEngine
{
	constexpr U32 MAX_PATH_LENGTH = 4096;

	typedef risStringNoEncoding risPath;

	template<class Allocator>
	risPath path_to_platform(StringId string_id, Allocator* allocator)
	{
		auto path = risPath(allocator, MAX_PATH_LENGTH);

		const auto internal_path = internal_string(string_id);
		for (U32 i = 0; internal_path[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (internal_path[i] == '/')
				path.put(L'\\');
			else
				path.put(internal_path[i]);
		}

		return path;
	}

	inline void path_to_platform(risPath path)
	{
		path.put(static_cast<CodePoint>(0));
		const auto buffer = path.get_buffer();

		for (U32 i = 0; buffer[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (buffer[i] == '/')
				buffer[i] = '\\';
		}
	}

	inline StringId path_to_ris(risPath& path)
	{
		risPath::Character encoded_path[MAX_PATH_LENGTH];
		path.get_encoded_string(encoded_path, MAX_PATH_LENGTH);

		for (U32 i = 0; encoded_path[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (encoded_path[i] == '\\')
				encoded_path[i] = '/';
		}

		return sid(encoded_path);
	}

}
