#include <cstdio>
#include <iostream>
#include <fstream>

#include "3rd_party/randomc/randomc.h"

#include "flags.h"
#include "risModules/risData/risString.h"
#include "risModules/risData/risEndian.h"
#include "risModules/risData/risAllocators.h"
#include "risModules/risData/risFlag.h"
#include "risModules/risData/risEncodings.h"
#include "risModules/risFile/risFiles.h"

using namespace ris;
using namespace risFile;

risFlag* flags;
risStackAllocator* stackAllocator;
CRandomMother* rng;

void test_flag();
void test_allocator();
void test_strings();
void test_ascii();
void test_file();
void test_file_and_unicode();
void test_risFile();
void test_rng();
void test_arguments(int argc, char* argv[]);
void test_endian();
void test_template();

int main(int argc, char *argv[])
{
	std::string cmd_command;


	std::cout << "Enter \"bruh\": ";
	std::cin >> cmd_command;

	std::cout << cmd_command << std::endl << (sid(cmd_command.c_str()) == sid("bruh")) << std::endl;

	// // startup
	// flags = new risFlag();
	// stackAllocator = new risStackAllocator(sizeof(U32) * 2);
	// rng = new CRandomMother(42);
	//
	// // tests
	// test_flag();
	// test_allocator();
	// test_strings();
	// test_ascii();
	// test_file();
	// test_file_and_unicode();
	// test_risFile();
	// test_rng();
	// test_arguments(argc, argv);
	// test_endian();
	// test_template();
	//
	//
	// // shutdown
	// delete rng;
	// delete stackAllocator;
	// delete flags;
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
	Marker marker = 0;

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
	
	const auto string_allocator = new risStackAllocator(sizeof(risStringBuffer<risUTF8<>>) + 256);
	auto string_buffer = static_cast<risStringBuffer<risUTF8<>>*>(string_allocator->alloc(sizeof(risStringBuffer<risUTF8<>>)));
	string_buffer->init(static_cast<risUTF8<>::Character*>(string_allocator->alloc(256)), 256);

	const auto input_values = new CodePoint[100];
	const auto encoded_values = new risUTF8<>::Character[100];
	const auto decoded_values = new CodePoint[100];
	
	for (U8 i = 0; i < 100; ++i)
	{
		//const auto random_value = static_cast<CodePoint>(rng->IRandom(0, 0x0010FFFF));

		input_values[i] = (0x0010FFFF - (i * 0x0010FFFF / 100)) % 0x0010FFFF;
		encoded_values[i] = 0;
		decoded_values[i] = 0;
	}

	string_buffer->put(input_values, 100);
	string_buffer->get_encoded_string(encoded_values, 100);
	string_buffer->get_decoded_string(decoded_values, 100);

	for (U8 i = 0; i < 100; ++i)
	{
		std::cout << input_values[i] << "=" << encoded_values[i] << "=" << decoded_values[i] << std::endl;
	}

	delete[] decoded_values;
	delete[] encoded_values;
	delete[] input_values;
	delete string_allocator;
}

void test_ascii()
{
	std::cout << "\nascii:" << std::endl;

	const auto string_allocator = new risStackAllocator(sizeof(risStringBuffer<risUTF8<>>) + 500);
	auto string_buffer = static_cast<risStringASCII*>(string_allocator->alloc(sizeof(risStringASCII)));
	string_buffer->init(static_cast<risStringASCII::Character*>(string_allocator->alloc(500)), 500);
	
	string_buffer->put("hoi").put(" ").put("poi");
	string_buffer->put(" ").format(true);
	string_buffer->put(" ").format(false);
	string_buffer->put(" ").format(0);
	string_buffer->put(" ").format(123456);
	string_buffer->put(" ").format(1513653123);
	string_buffer->put(" ").format(235235);
	string_buffer->put(" ").format(42);
	string_buffer->put(" ").format(1500008);
	string_buffer->put(" ").format(-13);
	string_buffer->put(" ").format(-987654321);
	string_buffer->put(" ").format(-0);
	string_buffer->put(" ").format(123.456f);
	string_buffer->put(" ").format(-24680.f);
	string_buffer->put(" ").format(-.0102030405f);

	auto result = new char[500];
	string_buffer->get_encoded_string(result, 500);

	std::cout << result << std::endl; // prints "hoi poi"

	delete[] result;
	delete string_allocator;
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
	// std::cout << "\nfile and unicode:" << std::endl;
	//
	// const auto stringAllocator = new risStackAllocator(sizeof(risStringBuffer) + 256);
	// auto sb = static_cast<risStringBuffer*>(stringAllocator->alloc(sizeof(risStringBuffer)));
	// sb->init(static_cast<U8*>(stringAllocator->alloc(256)), 256);
	//
	// sb->append_utf8('b');
	// sb->append_utf8('r');
	// sb->append_utf8('u');
	// sb->append_utf8('h');
	// sb->append_utf8(0x1F60D); // emoji with heart eyes
	// sb->append_utf8(0x2705); // green checkmark
	//
	// auto unicodeString = sb->get_string();
	//
	// std::ofstream writeFile;
	// writeFile.open("unicode_example.txt");
	// writeFile << unicodeString;
	// writeFile.close();
	//
	// delete stringAllocator;
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
	// init0(buffer, 100);
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
