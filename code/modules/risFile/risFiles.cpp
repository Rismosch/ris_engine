#include "pch.h"

#include "risFiles.h"

namespace risFile
{
#pragma region risWriteFile
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

	risWriteFile& risWriteFile::put(char value)
	{
		ofstream_.put(value);
		return *this;
	}

	risWriteFile& risWriteFile::write(const char* values, StreamSize count)
	{
		ofstream_.write(values, count);
		return *this;
	}

	StreamPosition risWriteFile::tellp()
	{
		return ofstream_.tellp();
	}

	risWriteFile& risWriteFile::seekp(StreamPosition offset, StreamLocation stream_location)
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

	risWriteFile& risWriteFile::flush()
	{
		ofstream_.flush();

		return *this;
	}
#pragma endregion

#pragma region risReadFile
	void risReadFile::open(const char* filename)
	{
		ifstream_.open(filename);
	}

	bool risReadFile::is_open() const
	{
		return ifstream_.is_open();
	}

	void risReadFile::close()
	{
		ifstream_.close();
	}

	StreamSize risReadFile::gcount()
	{
		return ifstream_.gcount();
	}

	risReadFile& risReadFile::get(char* buffer, StreamSize count)
	{
		ifstream_.get(buffer, count);

		return *this;
	}

	risReadFile& risReadFile::get(char* buffer, StreamSize count, char delim)
	{
		ifstream_.get(buffer, count, delim);

		return *this;
	}

	// risInStream& risReadFile::get(risOutStream& buffer, StreamSize count)
	// {
	// 	// figure this one out
	// 	return *this;
	// }
	//
	// risInStream& risReadFile::get(risOutStream& buffer, StreamSize count, char delim)
	// {
	// 	// figure this one out
	// 	return *this;
	// }

	risReadFile& risReadFile::ignore(StreamSize count, StreamCharacter delim)
	{
		ifstream_.ignore(count, delim);

		return *this;
	}

	StreamPosition risReadFile::tellg()
	{
		return ifstream_.tellg();
	}

	risReadFile& risReadFile::seekg(StreamPosition offset, StreamLocation stream_location)
	{
		switch (stream_location)
		{
		case StreamLocation::Beginning:
			ifstream_.seekg(offset, std::ios_base::beg);
			break;

		case StreamLocation::Current:
			ifstream_.seekg(offset, std::ios_base::cur);
			break;

		case StreamLocation::End:
			ifstream_.seekg(offset, std::ios_base::end);
			break;
		}

		return *this;
	}

	I32 risReadFile::sync()
	{
		return ifstream_.sync();
	}
#pragma endregion


}
