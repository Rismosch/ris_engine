#include "pch.h"

#include "risCompiler.h"

#include <fstream>
#include <tuple>
#include <filesystem>

#include "../../risEngine/risData/risAllocatorJanitors.h"
#include "../../risEngine/risData/risEncodings.h"

namespace risCompiler
{
	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr);

	risCompilerError compile_assets(risDoubleStackAllocator& allocator)
	{
		const auto alloc_jtr = risDoubleStackAllocatorJanitor(allocator);

		if (auto [err, path_utf8] = locate_asset_folder(alloc_jtr); err != risCompilerError::OK)
			return err;
		else
		{
			const auto path_utf16 = static_cast<wchar_t*>(alloc_jtr.allocator.alloc_front(static_cast<U32>(MAX_PATH_SIZE)));
			I32 root_path_length = convert<risUtf8, risUtf16LE>(path_utf8, path_utf16);

			if(!std::filesystem::is_directory(path_utf16))
				return risCompilerError::ASSET_FOLDER_MISSING;
			
			for (const auto& entry : std::filesystem::recursive_directory_iterator(path_utf16))
			{
				auto entry_path = entry.path();
				const std::ifstream asset_file(entry_path);
				if (!asset_file.good())
					continue;
			
				auto entry_string = entry_path.wstring();
				auto entry_extension = entry_path.extension();

				//TODO: strip asset folder redirect

				const auto asset_path_utf8 = static_cast<char*>(alloc_jtr.allocator.alloc_front(static_cast<U32>(MAX_PATH_SIZE)));
				convert<risUtf16LE, risUtf8>(entry_string.c_str(), asset_path_utf8, [](CodePoint code_point)
					{
						return code_point == static_cast<CodePoint>('\\')
							? static_cast<CodePoint>('/')
							: code_point;
					});

				auto test = asset_path_utf8;
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
		if (length > MAX_PATH_SIZE)
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
