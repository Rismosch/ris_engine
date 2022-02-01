#pragma once

#include "../risData/risPrimitives.h"
#include "../risData/risString.h"

namespace risEngine
{
	constexpr U32 MAX_PATH_LENGTH = 4096;

	typedef risStringASCII risPath;

	template<class Allocator>
	risPath* path_to_platform(StringId string_id, Allocator* allocator)
	{
		const auto path = allocator->alloc_class<risPath>();
		path->init(allocator, MAX_PATH_LENGTH);

		const auto internal_path = internal_string(string_id);
		for (U32 i = 0; internal_path[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (internal_path[i] == '/')
				path->put('\\');
			else
				path->put(internal_path[i]);
		}

		return path;
	}

	inline StringId path_to_ris(risPath* path_string)
	{
		risPath::Character encoded_path[MAX_PATH_LENGTH];
		path_string->get_encoded_string(encoded_path, MAX_PATH_LENGTH);

		for (U32 i = 0; encoded_path[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (encoded_path[i] == '\\')
				encoded_path[i] = '/';
		}

		return sid(encoded_path);
	}
}
