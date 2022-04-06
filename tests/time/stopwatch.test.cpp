#include "pch.h"

#include <risEngine/time/stopwatch.hpp>

#include <chrono>
#include <thread>

using namespace risEngine;

TEST(risStopwatchTests, ShouldReturnElapsed)
{
	const auto stopwatch = risStopwatch();

	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed1 = stopwatch.elapsed(risMilliSecond);
	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed2 = stopwatch.elapsed(risMilliSecond);
	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed3 = stopwatch.elapsed(risMilliSecond);

	EXPECT_GE(elapsed1, 100);
	EXPECT_LE(elapsed1, 200);
	EXPECT_GE(elapsed2, 200);
	EXPECT_LE(elapsed2, 300);
	EXPECT_GE(elapsed3, 300);
	EXPECT_LE(elapsed3, 400);
}

TEST(risStopwatchTests, ShouldReturnReset)
{
	auto stopwatch = risStopwatch();

	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed1 = stopwatch.reset(risMilliSecond);
	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed2 = stopwatch.reset(risMilliSecond);
	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed3 = stopwatch.reset(risMilliSecond);

	EXPECT_GE(elapsed1, 100);
	EXPECT_LE(elapsed1, 200);
	EXPECT_GE(elapsed2, 100);
	EXPECT_LE(elapsed2, 200);
	EXPECT_GE(elapsed3, 100);
	EXPECT_LE(elapsed3, 200);
}

TEST(risStopwatchTests, ShouldReturnCustomTimeUnit)
{
	auto stopwatch = risStopwatch();

	std::this_thread::sleep_for(std::chrono::milliseconds(100));
	const auto elapsed = stopwatch.elapsed();

	EXPECT_GE(elapsed, 60);
	EXPECT_LE(elapsed, 120);
}