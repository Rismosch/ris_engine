#include "pch.h"

#include <risEngine/data/template_utility.tpp>
#include <risEngine/engine/singleton_janitor.tpp>

using namespace risEngine;

std::vector<int> call_list;

struct SinlgetonParameters
{
	int number;
	SinlgetonParameters(int number) : number(number) { }
};

template <class T>
class SingletonMock
{
public: // singleton policy
	static SingletonMock& instance() { return instance_; }
	static void create(uintptr_t param)
	{
		instance_ = SingletonMock();
		instance_.sinlgeton_parameters = reinterpret_cast<SinlgetonParameters*>(param);

		call_list.push_back(instance_.sinlgeton_parameters->number);
	}
	static void destroy()
	{
		call_list.push_back(instance_.sinlgeton_parameters->number);
	}

private:
	SingletonMock(){}
	static SingletonMock instance_;

public:
	SinlgetonParameters* sinlgeton_parameters = nullptr;
};

SingletonMock<Int2Type<1>> SingletonMock<Int2Type<1>>::instance_;
SingletonMock<Int2Type<2>> SingletonMock<Int2Type<2>>::instance_;
SingletonMock<Int2Type<3>> SingletonMock<Int2Type<3>>::instance_;

class risSingletonJanitorTests : public ::testing::Test
{
protected:
	risSingletonJanitor* singleton_janitor = nullptr;
	int* calls = nullptr;
	

	void SetUp() override
	{
		singleton_janitor = new risSingletonJanitor();
		call_list.clear();
	}

	void TearDown() override
	{
		destroy_janitor();
	}

	void destroy_janitor()
	{
		if (singleton_janitor == nullptr)
			return;

		delete singleton_janitor;
		singleton_janitor = nullptr;
	}
};

TEST_F(risSingletonJanitorTests, ShouldCreateSingletonsWithParameters)
{
	auto parameters = SinlgetonParameters(1);
	singleton_janitor->create<SingletonMock<Int2Type<1>>>(reinterpret_cast<uintptr_t>(&parameters));
	
	EXPECT_EQ(parameters.number, SingletonMock<Int2Type<1>>::instance().sinlgeton_parameters->number);
	EXPECT_EQ(call_list.size(), 1);
}

TEST_F(risSingletonJanitorTests, ShouldCreateSingletonsInOrder)
{
	auto parameters1 = SinlgetonParameters(1);
	auto parameters2 = SinlgetonParameters(2);
	auto parameters3 = SinlgetonParameters(3);

	singleton_janitor->create<SingletonMock<Int2Type<1>>>(reinterpret_cast<uintptr_t>(&parameters1));
	singleton_janitor->create<SingletonMock<Int2Type<2>>>(reinterpret_cast<uintptr_t>(&parameters2));
	singleton_janitor->create<SingletonMock<Int2Type<3>>>(reinterpret_cast<uintptr_t>(&parameters3));

	ASSERT_EQ(call_list.size(), 3);
	EXPECT_EQ(call_list[0], 1);
	EXPECT_EQ(call_list[1], 2);
	EXPECT_EQ(call_list[2], 3);
}

TEST_F(risSingletonJanitorTests, ShouldDestroySingletonsInReversedOrder)
{
	auto parameters1 = SinlgetonParameters(1);
	auto parameters2 = SinlgetonParameters(2);
	auto parameters3 = SinlgetonParameters(3);

	singleton_janitor->create<SingletonMock<Int2Type<1>>>(reinterpret_cast<uintptr_t>(&parameters1));
	singleton_janitor->create<SingletonMock<Int2Type<2>>>(reinterpret_cast<uintptr_t>(&parameters2));
	singleton_janitor->create<SingletonMock<Int2Type<3>>>(reinterpret_cast<uintptr_t>(&parameters3));

	call_list.clear();

	destroy_janitor();

	ASSERT_EQ(call_list.size(), 3);
	EXPECT_EQ(call_list[0], 3);
	EXPECT_EQ(call_list[1], 2);
	EXPECT_EQ(call_list[2], 1);
}