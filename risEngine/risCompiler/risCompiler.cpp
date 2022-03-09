#include "pch.h"

#include "risCompiler.h"

#include <fstream>
#include <tuple>
#include <filesystem>

#include "../risData/risAllocatorJanitors.h"
#include "../risData/risEncodings.h"
#include "../risData/risString.h"

namespace risEngine
{
	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr);
	StringId get_asset_id(const risDoubleStackAllocatorJanitor& alloc_jtr, const wchar_t* full_path_utf16, const I32 root_path_length);

	risCompilerError compile_assets(risDoubleStackAllocator& allocator)
	{
		const auto alloc_jtr = risDoubleStackAllocatorJanitor(allocator);

		if (auto [err, path_utf8] = locate_asset_folder(alloc_jtr); err != risCompilerError::OK)
			return err;
		else
		{
			const auto path_utf16 = static_cast<wchar_t*>(alloc_jtr.allocator.alloc_front(static_cast<U32>(MAX_PATH_SIZE)));
			const I32 root_path_length = convert<risUtf8, risUtf16LE>(path_utf8, path_utf16);

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

				// TODO: check if extension can be compiled
				// TODO: if so, compute asset_id; save asset_id, path, extension and pointer to compiler in a struct; else skip

				auto asset_id = get_asset_id(alloc_jtr, entry_string.c_str(), root_path_length);

			}

			// TODO: sort assets by some criterium (asset_id, extension, whatever ? )
			// TODO: compute size package dictionary, see "Screenshot 2022-01-23 185119.png" in thoughts directory
			// TODO: foreach asset: compile and store at lowest file address, store asset_id, address and file_size in lowest directory entry
		}

		return risCompilerError::OK;
	}

	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr)
	{
		const std::ifstream redirect_file(ASSET_REDIRECT_PATH);
		if (!redirect_file.good())
			return { risCompilerError::REDIRECT_MISSING, {} };

		std::ifstream read_file;
		read_file.open(ASSET_REDIRECT_PATH);

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

	StringId get_asset_id(const risDoubleStackAllocatorJanitor& alloc_jtr, const wchar_t* full_path_utf16, const I32 root_path_length)
	{
		constexpr U32 max_wpath_size = MAX_PATH_SIZE / 2;

		const auto marker = alloc_jtr.allocator.get_marker_back();
		const auto stripped_asset_path_utf16 = static_cast<wchar_t*>(alloc_jtr.allocator.alloc_back(static_cast<U32>(MAX_PATH_SIZE)));
		const auto asset_path_utf8 = static_cast<char*>(alloc_jtr.allocator.alloc_back(static_cast<U32>(MAX_PATH_SIZE)));
		
		wcsncpy_s(stripped_asset_path_utf16, max_wpath_size,  &full_path_utf16[root_path_length], MAX_PATH_SIZE - root_path_length + 1);
		
		convert<risUtf16LE, risUtf8>(stripped_asset_path_utf16, asset_path_utf8, [](CodePoint code_point)
			{
				return code_point == static_cast<CodePoint>('\\')
					? static_cast<CodePoint>('/')
					: code_point;
			});

		const auto string_id = sid(asset_path_utf8);
		
		alloc_jtr.allocator.free_to_marker_back(marker);
		return string_id;
	}
}
