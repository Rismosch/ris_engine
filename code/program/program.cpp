#include <cstdio>
#include <vector>
#include <string>

#include "../3rd_party/rapidjson/writer.h"
#include "../3rd_party/rapidjson/reader.h"
#include "../3rd_party/randomc/randomc.h"

#include "flags.h"
#include "../modules/risData/crc32.h"
#include "../modules/risData/risString.h"
#include "../modules/risUtility/risLog.h"
#include "../modules/risUtility/risFlag.h"
#include "../modules/risUtility/risAllocator.h"

using namespace rapidjson;

using namespace ris;
using namespace risUtility;

risLog* logger;
risFlag* flags;
risAllocator* stackAllocator;
CRandomMother* rng;

void test_logger();
void test_flag();
void test_allocator();
void test_strings();
void test_rng();
void test_arguments(int argc, char* argv[]);
void test_json();

int main(int argc, char *argv[])
{
	// startup
	logger = new risLog(LogLevel::Warning);
	flags = new risFlag();
	stackAllocator = new risAllocator(sizeof(U32) * 2);
	rng = new CRandomMother(42);

	// tests
	test_logger();
	test_flag();
	test_allocator();
	test_strings();
	test_rng();
	test_arguments(argc, argv);
	test_json();

	// shutdown
	delete rng;
	delete stackAllocator;
	delete flags;
	delete logger;
}


void test_logger()
{
	std::cout << "\nlogger:" << std::endl;

	logger->trace("one");
	logger->debug("two");
	logger->warning("three");
	logger->error("four");

	// testing different logger...
	// https://stackoverflow.com/questions/41400/how-to-wrap-a-function-with-variable-length-arguments
	char buffer[50];
	int n, a = 5, b = 3;
	n = sprintf_s(buffer, "%d plus %d is %d", a, b, a + b);
	printf("[%s] is a string %d chars long\n", buffer, n);
}

void test_flag()
{
	std::cout << "\nflag:" << std::endl;

	flags->toggle(test0);
	flags->toggle(test2);

	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, true);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, false);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
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
}

void test_rng()
{
	std::cout << "\nrng:" << std::endl;

	for (U16 i = 0; i < 1000; ++i)
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
struct MyHandler {
	bool Null() { std::cout << "Null()" << std::endl; return true; }
	bool Bool(bool b) { std::cout << "Bool(" << std::boolalpha << b << ")" << std::endl; return true; }
	bool Int(int i) { std::cout << "Int(" << i << ")" << std::endl; return true; }
	bool Uint(unsigned u) { std::cout << "Uint(" << u << ")" << std::endl; return true; }
	bool Int64(int64_t i) { std::cout << "Int64(" << i << ")" << std::endl; return true; }
	bool Uint64(uint64_t u) { std::cout << "Uint64(" << u << ")" << std::endl; return true; }
	bool Double(double d) { std::cout << "Double(" << d << ")" << std::endl; return true; }
	bool RawNumber(const char* str, SizeType length, bool copy) {
		std::cout << "Number(" << str << ", " << length << ", " << std::boolalpha << copy << ")" << std::endl;
		return true;
	}
	bool String(const char* str, SizeType length, bool copy) {
		std::cout << "String(" << str << ", " << length << ", " << std::boolalpha << copy << ")" << std::endl;
		return true;
	}
	bool StartObject() { std::cout << "StartObject()" << std::endl; return true; }
	bool Key(const char* str, SizeType length, bool copy) {
		std::cout << "Key(" << str << ", " << length << ", " << std::boolalpha << copy << ", " << sid(str) << ")" << std::endl;
		return true;
	}
	bool EndObject(SizeType memberCount) { std::cout << "EndObject(" << memberCount << ")" << std::endl; return true; }
	bool StartArray() { std::cout << "StartArray()" << std::endl; return true; }
	bool EndArray(SizeType elementCount) { std::cout << "EndArray(" << elementCount << ")" << std::endl; return true; }
};

void test_json()
{
	std::cout << "\njson write:" << std::endl;

	StringBuffer sb;
	Writer<StringBuffer> writer(sb);

	writer.StartObject();

	writer.Key("some array");
	writer.StartArray();
	writer.Int(1);
	writer.Int(2);
	writer.Int(3);
	writer.EndArray();

	writer.Key("some object");
	writer.StartObject();
	writer.Key("my number");
	writer.Int(42);
	writer.Key("my bool");
	writer.Bool(false);
	writer.Key("my big number");
	writer.Int64(static_cast<int64_t>(1) << 33);
	writer.Key("my double");
	writer.Double(12.34);
	writer.EndObject();

	writer.EndObject();

	puts(sb.GetString());

	std::cout << "\njson read:" << std::endl;

	MyHandler handler;
	Reader reader;
	StringStream ss(sb.GetString());
	reader.Parse(ss, handler);
}