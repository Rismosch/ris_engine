#pragma once
#include <risEngine/engine/singleton_janitor.tpp>
#include <risEngine/data/primitives.hpp>

namespace risEngine
{
	struct risArguments
	{
		U32 job_threads;
	};

	class risEngine
	{
	public:
		risEngine(const risArguments& arguments);
		~risEngine();

		risEngine(const risEngine& other) = delete;
		risEngine(risEngine&& other) noexcept = delete;
		risEngine& operator=(const risEngine& other) = delete;
		risEngine& operator=(risEngine&& other) noexcept = delete;

		void run();

	private:
		risSingletonJanitor singleton_janitor_;
	};
}
