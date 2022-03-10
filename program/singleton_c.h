#pragma once
#include <iostream>

class Singleton_C
{
public:
	// singleton policy
	static Singleton_C* instance();
	static void create();
	static void destroy();

	// functions
	void print();

private:
	Singleton_C() = default;
	static Singleton_C* pInstance_;
};

Singleton_C* Singleton_C::pInstance_;

inline Singleton_C* Singleton_C::instance()
{
	return pInstance_;
}

inline void Singleton_C::create()
{
	std::cout << "create singleton c" << std::endl;
	if (!pInstance_)
		pInstance_ = new Singleton_C();
}

inline void Singleton_C::destroy()
{
	std::cout << "delete singleton c" << std::endl;
	delete pInstance_;
}

inline void Singleton_C::print()
{
	std::cout << "i am singleton c" << std::endl;
}
