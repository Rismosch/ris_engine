#pragma once

#include <fstream>

#include "risStreams.h"

namespace risStreams
{
	class risWriteFile : risOutStream
	{
	public:
		void open(const C8* filename);
		B is_open() const;
		void close();

		risOutStream& put(C8 value) override;
		risOutStream& write(const C8* values, U32 count) override;
		I64 tellp() override;
		risOutStream& seekp(I64 offset, StreamPosition stream_position = StreamPosition::Beginning) override;
		risOutStream& flush() override;

	private:
		std::ofstream ofstream_;
	};
}
