#include <risEngine/engine/engine.hpp>
#include <thread>

int main(int argc, char *argv[])
{
	const auto processor_count = std::thread::hardware_concurrency();

	auto arguments = risEngine::risArguments();
	arguments.job_threads = processor_count;

	auto engine = risEngine::risEngine(arguments);
	engine.run();
}