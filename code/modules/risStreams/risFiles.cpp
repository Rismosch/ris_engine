#include "pch.h"

#include "risFiles.h"

namespace risStreams
{
	void risWriteFile::open(const C8* filename)
	{
		ofstream_.open(filename);
	}

	B risWriteFile::is_open() const
	{
		return ofstream_.is_open();
	}

	void risWriteFile::close()
	{
		ofstream_.close();
	}

	risOutStream& risWriteFile::put(C8 value)
	{
		ofstream_.put(value);
		return *this;
	}

	risOutStream& risWriteFile::write(const C8* values, U32 count)
	{
		ofstream_.write(values, count);
		return *this;
	}

	I64 risWriteFile::tellp()
	{
		return ofstream_.tellp();
	}

	risOutStream& risWriteFile::seekp(I64 offset, StreamPosition stream_position)
	{
		switch (stream_position)
		{
		case StreamPosition::Beginning:
			ofstream_.seekp(offset, std::ios_base::beg);
			break;

		case StreamPosition::Current:
			ofstream_.seekp(offset, std::ios_base::cur);
			break;

		case StreamPosition::End:
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
