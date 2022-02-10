#pragma once
#include <cstdint>

namespace risEngine
{
	// unsigned int
	typedef uint8_t U8;		static_assert(sizeof(U8) == 1, "U8 must be exactly 8 bits in size");
	typedef uint16_t U16;	static_assert(sizeof(U16) == 2, "U16 must be exactly 16 bits in size");
	typedef uint32_t U32;	static_assert(sizeof(U32) == 4, "U32 must be exactly 32 bits in size");
	typedef uint64_t U64;	static_assert(sizeof(U64) == 8, "U64 must be exactly 64 bits in size");

	// signed int
	typedef int8_t I8;		static_assert(sizeof(I8) == 1, "I8 must be exactly 8 bits in size");
	typedef int16_t I16;	static_assert(sizeof(I16) == 2, "I16 must be exactly 16 bits in size");
	typedef int32_t I32;	static_assert(sizeof(I32) == 4, "I32 must be exactly 32 bits in size");
	typedef int64_t I64;	static_assert(sizeof(I64) == 8, "I64 must be exactly 64 bits in size");

	// float
	typedef float F32;		static_assert(sizeof(F32) == 4, "F32 must be exactly 32 bits in size");
	typedef double F64;		static_assert(sizeof(F64) == 8, "F64 must be exactly 64 bits in size");

	// fast 
	typedef uint_fast32_t U32F;	static_assert(sizeof(U32F) >= 4, "U32F must be at minimum 32 bits in size");
	typedef int_fast32_t I32F;	static_assert(sizeof(I32F) >= 4, "I32F must be at minimum 32 bits in size");

	// bool
	// any bool goes, really

	// char
	static_assert(sizeof(char) == 1, "CHAR must be exactly 16 bits in size");
}
