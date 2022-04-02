#include "pch.h"

#include <risEngine/data/primitives.hpp>
#include <risEngine/data/string.hpp>

using namespace risEngine;

class risStringTests : public ::testing::Test
{
protected:
	const char * string1 = "my string";
	const char * string2 = "brudda";
	const char * string3 = "was geht ab?";
	
	StringId string_id1 = string_id(string1);
	StringId string_id2 = string_id(string2);
	StringId string_id3 = string_id(string3);
};

TEST_F(risStringTests, ShouldReturnUniqueStringIds)
{
	EXPECT_NE(string_id1, string_id2);
	EXPECT_NE(string_id1, string_id3);
	EXPECT_NE(string_id2, string_id3);
}

TEST_F(risStringTests, ShouldReturnEqualIdForEqualStringRepeatedly)
{
	for (I32 i = 0; i < 100; ++i)
	{
		EXPECT_EQ(string_id1, string_id("my string"));
	}
}

TEST_F(risStringTests, ShouldReturnOriginalString)
{
	const auto original_string1 = internal_string(string_id1);
	const auto original_string2 = internal_string(string_id2);
	const auto original_string3 = internal_string(string_id3);

#if defined _DEBUG
	const auto left1 = std::string(string1);
	const auto left2 = std::string(string2);
	const auto left3 = std::string(string3);
	const auto right1 = std::string(original_string1);
	const auto right2 = std::string(original_string2);
	const auto right3 = std::string(original_string3);

	EXPECT_TRUE(left1 == right1);
	EXPECT_TRUE(left2 == right2);
	EXPECT_TRUE(left3 == right3);
#else
	EXPECT_EQ(original_string1, nullptr);
	EXPECT_EQ(original_string2, nullptr);
	EXPECT_EQ(original_string3, nullptr);
#endif
}