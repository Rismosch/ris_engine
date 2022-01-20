#pragma once

#include <fstream>

#include "risStreams.h"

namespace risStreams
{
	class risWriteFile : risOutStream
	{
	public:
		void open(const char* filename);
		bool is_open() const;
		void close();

		risOutStream& put(char value) override;
		risOutStream& write(const char* values, StreamSize count) override;
		StreamPosition tellp() override;
		risOutStream& seekp(StreamPosition offset, StreamLocation stream_location = StreamLocation::Beginning) override;
		risOutStream& flush() override;

	private:
		std::ofstream ofstream_;
	};
}
