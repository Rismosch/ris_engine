#include <cstdio>
#include <iostream>
#include <vector>
#include <fstream>

#include "../3rd_party/randomc/randomc.h"

#include "flags.h"
#include "../modules/risData/crc32.h"
#include "../modules/risData/risString.h"
#include "../modules/risData/risEndian.h"
#include "../modules/risData/risAllocator.h"
#include "../modules/risData/risDataUtility.h"
#include "../modules/risData/risFlag.h"
#include "../modules/risFile/risFiles.h"

using namespace ris;
using namespace risFile;

risFlag* flags;
risAllocator* stackAllocator;
CRandomMother* rng;

void test_flag();
void test_allocator();
void test_strings();
void test_file();
void test_file_and_unicode();
void test_risFile();
void test_rng();
void test_arguments(int argc, char* argv[]);
void test_endian();
void test_template();

int main(int argc, char *argv[])
{
	// startup
	flags = new risFlag();
	stackAllocator = new risAllocator(sizeof(U32) * 2);
	rng = new CRandomMother(42);

	// tests
	test_flag();
	test_allocator();
	test_strings();
	test_file();
	test_file_and_unicode();
	test_risFile();
	test_rng();
	test_arguments(argc, argv);
	test_endian();
	test_template();


	// shutdown
	delete rng;
	delete stackAllocator;
	delete flags;
}

void test_flag()
{
	std::cout << "\nflag:" << std::endl;

	flags->toggle(test0);
	flags->toggle(test2);

	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, true);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, false);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
	flags->apply(0x0123456789ABCDEF);
	std::cout << flags->to_string() << " Flag1: " << flags->get(test1) << std::endl;
}

void test_allocator()
{
	std::cout << "\nallocator:" << std::endl;

	U32* number0 = nullptr;
	U32* number1 = nullptr;
	U32* number2 = nullptr;
	U32* number3 = nullptr;
	risAllocator::Marker marker = 0;

	number0 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	*number0 = 42;

	marker = stackAllocator->get_marker();

	number1 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;
	*number1 = 13;
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;

	stackAllocator->free_to_marker(marker);

	number2 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;
	*number2 = 0;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;

	stackAllocator->clear();

	number3 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
	*number3 = 7;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
}

void test_strings()
{
	std::cout << "\nstrings:" << std::endl;

	auto stringid0 = sid("test1");
	auto stringid1 = sid("wazzup?");
	auto stringid2 = sid("bruh");

	std::cout << stringid0 << " " << stringid1 << " " << stringid2 << std::endl;

	auto string0 = internal_string(stringid0);
	auto string1 = internal_string(stringid1);
	auto string2 = internal_string(stringid2);

	if (string0 == nullptr)
		string0 = "null";
	if (string1 == nullptr)
		string1 = "null";
	if (string2 == nullptr)
		string2 = "null";

	std::cout << string0 << " " << string1 << " " << string2 << std::endl;

	std::cout << "shouldn't exist: " << (internal_string(static_cast<StringId>(42)) == nullptr) << " (there should be a 1)" << std::endl;

	std::cout << "\nstring buffer:" << std::endl;
	const auto stringAllocator = new risAllocator(sizeof(risStringBuffer) + 256);
	auto sb = static_cast<risStringBuffer*>(stringAllocator->alloc(sizeof(risStringBuffer)));
	sb->init(static_cast<U8*>(stringAllocator->alloc(256)), 256);

	sb->append('h');
	sb->append('e');
	sb->append('l');
	sb->append('l');
	sb->append('o');
	sb->append(' ');
	sb->append('w');
	sb->append('o');
	sb->append('r');
	sb->append('l');
	sb->append('d');

	std::cout << sb->get_string() << " " << sb->character_count() << " " << sb->size() << std::endl;

	sb->append(" bruh");

	std::cout << sb->get_string() << " " << sb->character_count() << " " << sb->size() << std::endl;

	sb->clear();

	std::cout << sb->get_string() << " " << sb->character_count() << " " << sb->size() << std::endl;


	U32* values = new U32[100];

	for (U8 i = 0; i < 100; ++i)
	{
		const U32 random_value = rng->IRandom(0, 0x0010FFFF);

		values[i] = random_value;
		sb->append_utf8(random_value);
	}

	const U32 count = sb->character_count(); // this will be significantly less than 100, because the buffer is too small and wont append further characters. This is by design.

	std::cout << "character count: " << count << std::endl;

	U32* decodedString = new U32[count];
	sb->decode_utf8(decodedString);

	for (U32 i = 0; i < count; ++i)
	{
		std::cout << values[i] << " = " << decodedString[i] << std::endl;
	}

	delete[] values;
	delete[] decodedString;

	delete stringAllocator;
}

void test_file()
{
	std::cout << "\nfile:" << std::endl;

	std::ofstream writeFile;
	writeFile.open("example.txt");
	writeFile << "hello world";
	writeFile.close();

	std::ifstream readFile;
	readFile.open("example.txt");

	char* buffer = new char[100]{};
	readFile.read(buffer, 100);
	std::cout << buffer << std::endl;
	readFile.close();
}

void test_file_and_unicode()
{
	std::cout << "\nfile and unicode:" << std::endl;

	const auto stringAllocator = new risAllocator(sizeof(risStringBuffer) + 256);
	auto sb = static_cast<risStringBuffer*>(stringAllocator->alloc(sizeof(risStringBuffer)));
	sb->init(static_cast<U8*>(stringAllocator->alloc(256)), 256);

	sb->append_utf8('b');
	sb->append_utf8('r');
	sb->append_utf8('u');
	sb->append_utf8('h');
	sb->append_utf8(0x1F60D); // emoji with heart eyes
	sb->append_utf8(0x2705); // green checkmark

	auto unicodeString = sb->get_string();

	std::ofstream writeFile;
	writeFile.open("unicode_example.txt");
	writeFile << unicodeString;
	writeFile.close();

	delete stringAllocator;
}

void test_risFile()
{
	std::cout << "\nrisFile Write:" << std::endl;

	risWriteFile writeFile;
	writeFile.open("test.txt");
	writeFile.write("this is an apple", 16);
	auto pos = writeFile.tellp();
	writeFile.seekp(pos - 7);
	writeFile.write(" sam", 4);
	writeFile.seekp(-8, StreamLocation::End);
	writeFile.write("t", 1);
	writeFile.close();

	std::cout << "\nrisFile Read:" << std::endl;
	risReadFile risReadFile;
	risReadFile.open("test.txt");
	char* buffer = new char[100];
	init0(buffer, 100);
	risReadFile.get(buffer, 100);
	std::cout << buffer << std::endl;
	risReadFile.close();
}

void test_rng()
{
	std::cout << "\nrng:" << std::endl;

	for (U16 i = 0; i < 10; ++i)
	{
		std::cout << rng->BRandom() << " " << rng->Random() << " " << rng->IRandom(-24, 13) << std::endl;
	}
}

void test_arguments(int argc, char* argv[])
{
	std::cout << "\narguments:" << std::endl;

	for (int i = 0; i < argc; ++i)
	{
		std::cout << argv[i] << std::endl;
	}
}

void test_endian()
{
	std::cout << "\nendian:" << std::endl;

	U16 value1 = 0x00FF;
	U32 value2 = 0x00FF00FF;
	F32 value3 = convertU32(value2);

	flags->apply(value1);
	std::cout << flags->to_string() << std::endl;
	flags->apply(value2);
	std::cout << flags->to_string() << std::endl;

	auto result1 = swapU16(value1);
	auto result2 = swapU32(value2);
	auto result3 = swapF32(value3);

	std::cout << result1 << std::endl;
	std::cout << result2 << std::endl;
	std::cout << result3 << std::endl;

	flags->apply(result1);
	std::cout << flags->to_string() << std::endl;
	flags->apply(result2);
	std::cout << flags->to_string() << std::endl;
	flags->apply(convertF32(result3));
	std::cout << flags->to_string() << std::endl;
}

class PrinterA
{
public:
	void print() { std::cout << "A" << std::endl; }
};

class PrinterB
{
public:
	void print() { std::cout << "B" << std::endl; }
};

class PrinterC
{
public:
	void print() { std::cout << "C" << std::endl; }
};

class PrinterD
{
	
};

template<typename Printer>
void print(Printer printer)
{
	printer.print();
}

void test_template()
{
	std::cout << "\ntemplate:" << std::endl;

	PrinterA a;
	PrinterB b;
	PrinterC c;
	PrinterD d;

	print(a);
	print(b);
	print(c);
	// print(d); // this does not compile
}
