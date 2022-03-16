#include "../risEngine/risEngine/risEngine.h"
#include <thread>

int main(int argc, char *argv[])
{
	const auto processor_count = std::thread::hardware_concurrency();

	auto arguments = risEngine::risArguments();
	arguments.job_threads = processor_count;

	auto engine = risEngine::risEngine(arguments);
	engine.run();

	// stack_allocator.init(1000000);
	//
	// test_allocator();
	// test_strings();
	// // test_file();
	// test_resource_compiler();
	// test_rng();
	// test_arguments(argc, argv);
	// test_singleton();
	//
	// stack_allocator.release();
}

// void test_allocator()
// {
// 	std::cout << "\nallocator:" << std::endl;
//
// 	U32* number0 = nullptr;
// 	U32* number1 = nullptr;
// 	U32* number2 = nullptr;
// 	U32* number3 = nullptr;
// 	Marker marker = 0;
//
// 	number0 = static_cast<U32*>(stack_allocator.alloc(sizeof(U32)));
// 	*number0 = 42;
//
// 	marker = stack_allocator.get_marker();
//
// 	number1 = static_cast<U32*>(stack_allocator.alloc(sizeof(U32)));
// 	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;
// 	*number1 = 13;
// 	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;
//
// 	stack_allocator.free_to_marker(marker);
//
// 	number2 = static_cast<U32*>(stack_allocator.alloc(sizeof(U32)));
// 	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;
// 	*number2 = 0;
// 	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;
//
// 	stack_allocator.clear();
//
// 	number3 = static_cast<U32*>(stack_allocator.alloc(sizeof(U32)));
// 	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
// 	*number3 = 7;
// 	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
// }
//
// void test_strings()
// {
// 	std::cout << "\nstrings:" << std::endl;
//
// 	auto stringid0 = sid("test1");
// 	auto stringid1 = sid("wazzup?");
// 	auto stringid2 = sid("bruh");
//
// 	std::cout << stringid0 << " " << stringid1 << " " << stringid2 << std::endl;
//
// 	auto string0 = internal_string(stringid0);
// 	auto string1 = internal_string(stringid1);
// 	auto string2 = internal_string(stringid2);
//
// 	if (string0 == nullptr)
// 		string0 = "null";
// 	if (string1 == nullptr)
// 		string1 = "null";
// 	if (string2 == nullptr)
// 		string2 = "null";
//
// 	std::cout << string0 << " " << string1 << " " << string2 << std::endl;
//
// 	std::cout << "shouldn't exist: " << (internal_string(static_cast<StringId>(42)) == nullptr) << " (there should be a 1)" << std::endl;
//
//
// 	const char* c_str_0 = "hello world";
// 	char* c_str_1 = new char[12]{ "hello world" };
//
// 	auto c_sid0 = sid(c_str_0);
// 	auto c_sid1 = sid(c_str_1);
//
// 	std::cout << c_sid0  << " == "  << c_sid1 << " : " << (c_sid0 == c_sid1) << std::endl;
// }
//
// void test_file()
// {
// 	std::cout << "\nfile:" << std::endl;
//
// 	std::ofstream writeFile;
// 	writeFile.open("example.txt");
// 	writeFile << "hello world";
// 	writeFile.close();
//
// 	std::ifstream readFile;
// 	readFile.open("example.txt");
//
// 	char* buffer = new char[100]{};
// 	readFile.read(buffer, 100);
// 	std::cout << buffer << std::endl;
// 	readFile.close();
// }
//
// void test_resource_compiler()
// {
// 	std::cout << "\nresource compiler:" << std::endl;
// 	risDoubleStackAllocator double_stack_allocator;
// 	double_stack_allocator.init(1000000);
// 	
// 	auto error = compile_assets(double_stack_allocator);
// 	
// 	double_stack_allocator.release();
// }
//
// void test_rng()
// {
// 	std::cout << "\nrng:" << std::endl;
//
// 	for (U16 i = 0; i < 10; ++i)
// 	{
// 		std::cout << rng.BRandom() << " " << rng.Random() << " " << rng.IRandom(-24, 13) << std::endl;
// 	}
// }
//
// void test_arguments(int argc, char* argv[])
// {
// 	std::cout << "\narguments:" << std::endl;
//
// 	for (int i = 0; i < argc; ++i)
// 	{
// 		std::cout << argv[i] << std::endl;
// 	}
// }
//
// // class Singleton_B
// // {
// // public:
// // 	// singleton policy
// // 	static Singleton_B* instance()
// // 	{
// // 		return instance_;
// // 	}
// //
// // 	static void create()
// // 	{
// // 		std::cout << "create singleton b" << std::endl;
// // 		if (!instance_)
// // 			instance_ = new Singleton_B;
// // 	}
// //
// //
// // 	static void destroy()
// // 	{
// // 		std::cout << "destroy singleton b" << std::endl;
// // 		delete instance_;
// // 	}
// //
// // 	// functions
// // 	void print()
// // 	{
// // 		std::cout << "i am singleton b" << std::endl;
// // 	}
// //
// // private:
// // 	static Singleton_B* instance_;
// // };
//
// void test_singleton()
// {
// 	std::cout << "singleton janitor:" << std::endl;
//
// 	risSingletonJanitor jtr = risSingletonJanitor();
//
// 	jtr.setup<Singleton_A>();
// 	jtr.setup<Singleton_B>();
// 	jtr.setup<Singleton_C>();
//
// 	Singleton_A::instance()->print();
// 	Singleton_B::instance()->print();
// 	Singleton_C::instance()->print();
// }


