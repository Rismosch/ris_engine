#pragma once

#include "../risData/risStreams.h"
#include <fstream>

namespace risFile
{
	using namespace risData;

	class risWriteFile
	{
	public:
		void open(const char* filename);
		bool is_open() const;
		void close();

		risWriteFile& put(char value);
		risWriteFile& write(const char* values, StreamSize count);

		StreamPosition tellp();
		risWriteFile& seekp(StreamPosition offset, StreamLocation stream_location = StreamLocation::Beginning);
		risWriteFile& flush();

	private:
		std::ofstream ofstream_;
	};

	class risReadFile
	{
	public:
		void open(const char* filename);
		bool is_open() const;
		void close();

		StreamSize gcount();

		risReadFile& get(char* buffer, StreamSize count);
		risReadFile& get(char* buffer, StreamSize count, char delim);
		// risReadFile& get(risOutStream& buffer, StreamSize count);
		// risReadFile& get(risOutStream& buffer, StreamSize count, char delim);

		risReadFile& ignore(StreamSize count = 1, StreamCharacter delim = EndOfFile);

		StreamPosition tellg();
		risReadFile& seekg(StreamPosition offset, StreamLocation stream_location);
		I32 sync();

	private:
		std::ifstream ifstream_;
	};
}
