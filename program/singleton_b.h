#pragma once
#include <iostream>

class Singleton_B
{
public:
	// singleton policy
	static Singleton_B* instance();
	static void create();
	static void destroy();

	// functions
	void print();

private:
	Singleton_B() = default;
	static Singleton_B* pInstance_;
};

Singleton_B* Singleton_B::pInstance_;

inline Singleton_B* Singleton_B::instance()
{
	return pInstance_;
}

inline void Singleton_B::create()
{
	std::cout << "create singleton b" << std::endl;
	if (!pInstance_)
		pInstance_ = new Singleton_B();
}

inline void Singleton_B::destroy()
{
	std::cout << "delete singleton b" << std::endl;
	delete pInstance_;
}

inline void Singleton_B::print()
{
	std::cout << "i am singleton b" << std::endl;
}
