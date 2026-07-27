#include "greeter.hpp"
#include <stdio.h>


void Greeter::greet() {
#ifdef NDEBUG
	printf("i was built without debug!\n");
    std::cout << "i was built without debug!\n";
#else
	printf("hello :)\n");
#endif
}
