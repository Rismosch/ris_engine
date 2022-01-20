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

	class risReadFile : risInStream
	{
	public:
		void open(const char* filename);
		bool is_open() const;
		void close();

		StreamSize gcount() override;

		risInStream& get(char* buffer, StreamSize count) override;
		risInStream& get(char* buffer, StreamSize count, char delim) override;
		risInStream& get(risOutStream& buffer, StreamSize count) override;
		risInStream& get(risOutStream& buffer, StreamSize count, char delim) override;

		risInStream& ignore(StreamSize count = 1, StreamCharacter delim = EndOfFile) override;

		StreamPosition tellg() override;
		risInStream& seekg(StreamPosition offset, StreamLocation stream_location) override;
		I32 sync() override;

	private:
		std::ifstream ifstream_;
	};
}
