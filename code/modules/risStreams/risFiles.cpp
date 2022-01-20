#include "pch.h"

#include "risFiles.h"

namespace risStreams
{
	void risWriteFile::open(const char* filename)
	{
		ofstream_.open(filename);
	}

	bool risWriteFile::is_open() const
	{
		return ofstream_.is_open();
	}

	void risWriteFile::close()
	{
		ofstream_.close();
	}

	risOutStream& risWriteFile::put(char value)
	{
		ofstream_.put(value);
		return *this;
	}

	risOutStream& risWriteFile::write(const char* values, StreamSize count)
	{
		ofstream_.write(values, count);
		return *this;
	}

	StreamPosition risWriteFile::tellp()
	{
		return ofstream_.tellp();
	}

	risOutStream& risWriteFile::seekp(StreamPosition offset, StreamLocation stream_location)
	{
		switch (stream_location)
		{
		case StreamLocation::Beginning:
			ofstream_.seekp(offset, std::ios_base::beg);
			break;

		case StreamLocation::Current:
			ofstream_.seekp(offset, std::ios_base::cur);
			break;

		case StreamLocation::End:
			ofstream_.seekp(offset, std::ios_base::end);
			break;
		}

		return *this;
	}

	risOutStream& risWriteFile::flush()
	{
		ofstream_.flush();

		return *this;
	}
}
