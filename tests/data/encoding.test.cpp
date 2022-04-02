#include "pch.h"

#include <risEngine/data/encoding.hpp>

#define STRING_LENGTH 256

using namespace risEngine;

TEST(risEncodingTests, ShouldEncodeAndDecodeUTF8)
{
	const char* original = "actually, this is ascii, because i don't think cpp has utf8 strings :P";
	auto copy = new char[STRING_LENGTH];

	convert_encoding<risUtf8, risUtf8>(original, copy);

	auto left = std::string(original);
	auto right = std::string(copy);

	EXPECT_TRUE(left == right);

	delete[] copy;
}

TEST(risEncodingTests, ShouldEncodeAndDecodeUTF16LE)
{
	const wchar_t* original = L"now we're talking! still ascii though, cuz reasons";
	auto copy = new wchar_t[STRING_LENGTH];

	convert_encoding<risUtf16LE, risUtf16LE>(original, copy);

	auto left = std::wstring(original);
	auto right = std::wstring(copy);

	EXPECT_TRUE(left == right);

	delete[] copy;
}

TEST(risEncodingTests, ShouldConvertFromUtf8ToUTF16LE)
{
	const char* original = "hello world";
	auto copy = new wchar_t[STRING_LENGTH];

	convert_encoding<risUtf8, risUtf16LE>(original, copy);

	auto left = std::wstring(L"hello world");
	auto right = std::wstring(copy);

	EXPECT_TRUE(left == right);

	delete[] copy;
}

TEST(risEncodingTests, ShouldConvertFromUtf16LEToUTF8)
{
	const wchar_t* original = L"hello world";
	auto copy = new char[STRING_LENGTH];

	convert_encoding<risUtf16LE, risUtf8>(original, copy);

	auto left = std::string("hello world");
	auto right = std::string(copy);

	EXPECT_TRUE(left == right);

	delete[] copy;
}

TEST(risEncodingTests, ShouldReplaceCharacters)
{
	const wchar_t* original = L"hello world";
	auto copy = new wchar_t[STRING_LENGTH];

	convert_encoding<risUtf16LE, risUtf16LE>(original, copy, [](wchar_t c) {return c == L'l' ? L'p' : c; });

	auto left = std::wstring(L"heppo worpd");
	auto right = std::wstring(copy);

	EXPECT_TRUE(left == right);

	delete[] copy;
}