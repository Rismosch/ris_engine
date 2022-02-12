#include "pch.h"

#include "risCompiler.h"

#include <fstream>
#include <tuple>
#include <filesystem>
#include <string>

#include "../../risEngine/risData/risAllocatorJanitors.h"
#include "../../risEngine/risData/risEncodings.h"

namespace risCompiler
{
	std::tuple<risCompilerError, char*> locate_asset_folder(const risDoubleStackAllocatorJanitor& alloc_jtr);
	wchar_t* convert_utf8_path_to_utf16(const risDoubleStackAllocatorJanitor& alloc_jtr, const char* utf8_asset_path);

	risCompilerError compile_assets(risDoubleStackAllocator& allocator)
	{
		const auto alloc_jtr = risDoubleStackAllocatorJanitor(allocator);

		if (auto [err, path] = locate_asset_folder(alloc_jtr); err != risCompilerError::OK)
			return err;
		else
		{
			const auto asset_path = convert_utf8_path_to_utf16(alloc_jtr, path);

			if(!std::filesystem::is_directory(asset_path))
				return risCompilerError::ASSET_FOLDER_MISSING;

			for (const auto& entry : std::filesystem::recursive_directory_iterator(asset_path))
			{
				auto entry_path = entry.path();
				const std::ifstream asset_file(entry_path);
				if (!asset_file.good())
					continue;

				auto entry_string = entry_path.wstring();
				//TODO: strip asset folder redirect
				//TODO: change \ to /

				auto entry_extension = entry_path.extension();
				auto test = entry_extension;
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

	wchar_t* convert_utf8_path_to_utf16(const risDoubleStackAllocatorJanitor& alloc_jtr, const char* utf8_asset_path)
	{
		const auto utf16_asset_path = static_cast<wchar_t*>(alloc_jtr.allocator.alloc_front(static_cast<U32>(MAX_PATH_SIZE)));
		I32 i = 0, j = 0;

		while (utf8_asset_path[i] != 0)
		{
			auto input_lambda = [&]
			{
				return utf8_asset_path[i++];
			};
			auto output_lambda = [&](risUtf16LE::Character input)
			{
				utf16_asset_path[j++] = input;
			};

			const auto code_point = risUtf8::decode(input_lambda);
			risUtf16LE::encode(code_point, output_lambda);
		}
		utf16_asset_path[j] = 0;

		return utf16_asset_path;
	}
}
