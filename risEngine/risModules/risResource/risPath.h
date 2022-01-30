#pragma once

#include "../risData/risPrimitives.h"
#include "../risData/risString.h"

namespace risResource
{
	using namespace risData;

	constexpr U32 MAX_PATH_LENGTH = 4096;

	typedef risStringASCII risPathString;

	template<class Allocator>
	risPathString* path_to_platform(StringId string_id, Allocator* allocator)
	{
		const auto path = static_cast<risPathString*>(allocator->alloc(sizeof(risPathString)));
		path->init(static_cast<risPathString::Character*>(allocator->alloc(MAX_PATH_LENGTH)), MAX_PATH_LENGTH);

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

	inline StringId path_to_ris(risPathString* path_string)
	{
		risPathString::Character encoded_path[MAX_PATH_LENGTH];
		path_string->get_encoded_string(encoded_path, MAX_PATH_LENGTH);

		for (U32 i = 0; encoded_path[i] != 0 && i < MAX_PATH_LENGTH; ++i)
		{
			if (encoded_path[i] == '\\')
				encoded_path[i] = '/';
		}

		return sid(encoded_path);
	}
}
