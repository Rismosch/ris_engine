#include "pch.h"

#include "risCompiler.h"

#include <fstream>
#include <tuple>
#include <filesystem>

#include "../../risEngine/risData/risAllocatorJanitors.h"

namespace risCompiler
{
	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr);

	risCompilerError compile_assets(risDoubleStackAllocator& allocator)
	{
		const auto alloc_jtr = risDoubleStackAllocatorJanitor(allocator);

		if (auto [err, path] = locate_asset_folder(alloc_jtr); err != risCompilerError::OK)
			return err;
		else
		{
			if(!std::filesystem::is_directory(path))
			{
				return risCompilerError::ASSET_FOLDER_MISSING;
			}

			for (const auto& entry : std::filesystem::recursive_directory_iterator(path))
			{
				auto entry_path = entry.path();
				const std::ifstream asset_file(entry_path);
				if (!asset_file.good())
					continue;


			}
		}

		return risCompilerError::OK;
	}

	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr)
	{
		const std::ifstream redirect_file(RESOURCE_REDIRECT_PATH);
		if (!redirect_file.good())
			return { risCompilerError::REDIRECT_MISSING, {} };

		std::ifstream read_file;
		read_file.open(RESOURCE_REDIRECT_PATH);

		// get length
		read_file.seekg(0, std::ios_base::end);
		const auto length = static_cast<I32>(read_file.tellg());
		if (length > MAX_PATH_LENGTH)
		{
			read_file.close();
			return { risCompilerError::REDIRECT_PATH_TOO_LONG, {} };
		}

		read_file.seekg(0);

		// read path
		const auto path = static_cast<char*>(alloc_jtr.allocator.alloc(length + 1));
		read_file.read(path, length);
		path[length] = 0;

		read_file.close();

		return { risCompilerError::OK, path };
	}
}
