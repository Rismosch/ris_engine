#pragma once

#include <functional>
#include <stack>

namespace risEngine
{
	class risSingletonJanitor
	{
	public:
		template<class Singleton>
		void setup()
		{
			Singleton::create();
			destroy_functions_.push(Singleton::destroy);
		}

		~risSingletonJanitor()
		{
			while (!destroy_functions_.empty())
			{
				destroy_functions_.top().operator()();
				destroy_functions_.pop();
			}
		}

		risSingletonJanitor() = default;

		risSingletonJanitor(const risSingletonJanitor& other) = delete;
		risSingletonJanitor(risSingletonJanitor&& other) noexcept = delete;
		risSingletonJanitor& operator=(const risSingletonJanitor& other) = delete;
		risSingletonJanitor& operator=(risSingletonJanitor&& other) noexcept = delete;

	private:
		std::stack<std::function<void()>> destroy_functions_;
	};
}
