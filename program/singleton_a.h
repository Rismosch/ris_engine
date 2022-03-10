#pragma once
#include <iostream>

class Singleton_A
{
public:
	// singleton policy
	static Singleton_A* instance();
	static void create();
	static void destroy();

	// functions
	void print();

private:
	Singleton_A() = default;
	static Singleton_A* pInstance_;
};

Singleton_A* Singleton_A::pInstance_;

inline Singleton_A* Singleton_A::instance()
{
	return pInstance_;
}

inline void Singleton_A::create()
{
	std::cout << "create singleton a" << std::endl;
	if (!pInstance_)
	pInstance_ = new Singleton_A();
}

inline void Singleton_A::destroy()
{
	std::cout << "delete singleton a" << std::endl;
	delete pInstance_;
}

inline void Singleton_A::print()
{
	std::cout << "i am singleton a" << std::endl;
}
