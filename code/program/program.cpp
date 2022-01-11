#include <cstdio>
#include <vector>
#include <string>

#include "../3rd_party/rapidjson/prettywriter.h"

#include "flags.h"
#include "../modules/risData/crc32.h"
#include "../modules/risData/risString.h"
#include "../modules/risUtility/risLog.h"
#include "../modules/risUtility/risFlag.h"
#include "../modules/risUtility/risAllocator.h"
#include "../modules/risUtility/risRandom.h"

using namespace rapidjson;

using namespace ris;
using namespace risUtility;

risLog* logger;
risFlag* flags;
risAllocator* stackAllocator;
risRandom* rng;

void test_logger();
void test_flag();
void test_allocator();
void test_strings();
void test_rng();
void test_arguments(int argc, char* argv[]);
void test_json();

int main(int argc, char *argv[])
{
	// startup
	logger = new risLog(LogLevel::Warning);
	flags = new risFlag();
	stackAllocator = new risAllocator(sizeof(U32) * 2);
	rng = new risRandom(42);

	// tests
	test_logger();
	test_flag();
	test_allocator();
	test_strings();
	// test_rng();
	test_arguments(argc, argv);
	test_json();

	// shutdown
	delete rng;
	delete stackAllocator;
	delete flags;
	delete logger;
}


void test_logger()
{
	std::cout << "\nlogger:" << std::endl;

	logger->trace("one");
	logger->debug("two");
	logger->warning("three");
	logger->error("four");
}

void test_flag()
{
	std::cout << "\nflag:" << std::endl;

	flags->toggle(test0);
	flags->toggle(test2);

	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, true);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->set(test1, false);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test1);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
	flags->toggle(test2);
	std::cout << flags->toString() << " Flag1: " << flags->get(test1) << std::endl;
}

void test_allocator()
{
	std::cout << "\nallocator:" << std::endl;

	U32* number0 = nullptr;
	U32* number1 = nullptr;
	U32* number2 = nullptr;
	U32* number3 = nullptr;
	risAllocator::Marker marker = 0;

	number0 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	*number0 = 42;

	marker = stackAllocator->get_marker();

	number1 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;
	*number1 = 13;
	std::cout << *number0 << "\t" << *number1 << "\t0\t0" << std::endl;

	stackAllocator->free_to_marker(marker);

	number2 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;
	*number2 = 0;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t0" << std::endl;

	stackAllocator->clear();

	number3 = static_cast<U32*>(stackAllocator->alloc(sizeof(U32)));
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
	*number3 = 7;
	std::cout << *number0 << "\t" << *number1 << "\t" << *number2 << "\t" << *number3 << std::endl;
}

void test_strings()
{
	std::cout << "\nstrings:" << std::endl;

	auto stringid0 = sid("test1");
	auto stringid1 = sid("wazzup?");
	auto stringid2 = sid("bruh");

	std::cout << stringid0 << " " << stringid1 << " " << stringid2 << std::endl;

	auto string0 = internal_string(stringid0);
	auto string1 = internal_string(stringid1);
	auto string2 = internal_string(stringid2);

	if (string0 == nullptr)
		string0 = "null";
	if (string1 == nullptr)
		string1 = "null";
	if (string2 == nullptr)
		string2 = "null";

	std::cout << string0 << " " << string1 << " " << string2 << std::endl;

	std::cout << "shouldn't exist: " << (internal_string(static_cast<StringId>(42)) == nullptr) << " (there should be a 1)" << std::endl;
}

void test_rng()
{
	std::cout << "\nrng:" << std::endl;

	for (U16 i = 0; i < 1000; ++i)
	{
		std::cout << rng->bRandom() << " " << rng->fRandom() << " " << rng->iRandom(-24, 13) << std::endl;
	}
}

void test_arguments(int argc, char* argv[])
{
	std::cout << "\narguments:" << std::endl;

	for (int i = 0; i < argc; ++i)
	{
		std::cout << argv[i] << std::endl;
	}
}

#pragma region json_classes

class Person {
public:
	Person(const std::string& name, unsigned age) : name_(name), age_(age) {}
	Person(const Person& rhs) : name_(rhs.name_), age_(rhs.age_) {}
	virtual ~Person();

	Person& operator=(const Person& rhs) {
		name_ = rhs.name_;
		age_ = rhs.age_;
		return *this;
	}

protected:
	template <typename Writer>
	void Serialize(Writer& writer) const {
		// This base class just write out name-value pairs, without wrapping within an object.
		writer.String("name");
#if RAPIDJSON_HAS_STDSTRING
		writer.String(name_);
#else
		writer.String(name_.c_str(), static_cast<SizeType>(name_.length())); // Supplying length of string is faster.
#endif
		writer.String("age");
		writer.Uint(age_);
	}

private:
	std::string name_;
	unsigned age_;
};

Person::~Person() {
}

class Education {
public:
	Education(const std::string& school, double GPA) : school_(school), GPA_(GPA) {}
	Education(const Education& rhs) : school_(rhs.school_), GPA_(rhs.GPA_) {}

	template <typename Writer>
	void Serialize(Writer& writer) const {
		writer.StartObject();

		writer.String("school");
#if RAPIDJSON_HAS_STDSTRING
		writer.String(school_);
#else
		writer.String(school_.c_str(), static_cast<SizeType>(school_.length()));
#endif

		writer.String("GPA");
		writer.Double(GPA_);

		writer.EndObject();
	}

private:
	std::string school_;
	double GPA_;
};

class Dependent : public Person {
public:
	Dependent(const std::string& name, unsigned age, Education* education = 0) : Person(name, age), education_(education) {}
	Dependent(const Dependent& rhs) : Person(rhs), education_(0) { education_ = (rhs.education_ == 0) ? 0 : new Education(*rhs.education_); }
	virtual ~Dependent();

	Dependent& operator=(const Dependent& rhs) {
		if (this == &rhs)
			return *this;
		delete education_;
		education_ = (rhs.education_ == 0) ? 0 : new Education(*rhs.education_);
		return *this;
	}

	template <typename Writer>
	void Serialize(Writer& writer) const {
		writer.StartObject();

		Person::Serialize(writer);

		writer.String("education");
		if (education_)
			education_->Serialize(writer);
		else
			writer.Null();

		writer.EndObject();
	}

private:

	Education* education_;
};

Dependent::~Dependent() {
	delete education_;
}

class Employee : public Person {
public:
	Employee(const std::string& name, unsigned age, bool married) : Person(name, age), dependents_(), married_(married) {}
	Employee(const Employee& rhs) : Person(rhs), dependents_(rhs.dependents_), married_(rhs.married_) {}
	virtual ~Employee();

	Employee& operator=(const Employee& rhs) {
		static_cast<Person&>(*this) = rhs;
		dependents_ = rhs.dependents_;
		married_ = rhs.married_;
		return *this;
	}

	void AddDependent(const Dependent& dependent) {
		dependents_.push_back(dependent);
	}

	template <typename Writer>
	void Serialize(Writer& writer) const {
		writer.StartObject();

		Person::Serialize(writer);

		writer.String("married");
		writer.Bool(married_);

		writer.String(("dependents"));
		writer.StartArray();
		for (std::vector<Dependent>::const_iterator dependentItr = dependents_.begin(); dependentItr != dependents_.end(); ++dependentItr)
			dependentItr->Serialize(writer);
		writer.EndArray();

		writer.EndObject();
	}

private:
	std::vector<Dependent> dependents_;
	bool married_;
};

Employee::~Employee() {
}

#pragma endregion

void test_json()
{
	std::cout << "\njson:" << std::endl;

	std::vector<Employee> employees;

	employees.push_back(Employee("Milo YIP", 34, true));
	employees.back().AddDependent(Dependent("Lua YIP", 3, new Education("Happy Kindergarten", 3.5)));
	employees.back().AddDependent(Dependent("Mio YIP", 1));

	employees.push_back(Employee("Percy TSE", 30, false));

	StringBuffer sb;
	PrettyWriter<StringBuffer> writer(sb);

	writer.StartArray();
	for (std::vector<Employee>::const_iterator employeeItr = employees.begin(); employeeItr != employees.end(); ++employeeItr)
		employeeItr->Serialize(writer);
	writer.EndArray();

	puts(sb.GetString());
}