#pragma once

namespace risFile
{
	class risPath
	{
	public:
		template<typename Allocator>
		risPath(Allocator allocator);

	private:
		bool is_directory_;
	};
}