/*
 * Mother-of-All Pseudorandom Number Generators, slightly adapted for my purposes.
 *
 * Original Author Details:
 * Author:        Agner Fog
 * Date created:  1997
 * Last modified: 2008-11-16
 * Project:       randomc.h
 * Source URL:    www.agner.org/random
 * 
 * This is a multiply-with-carry type of random number generator
 * invented by George Marsaglia.  The algorithm is:             
 * S = 2111111111*X[n-4] + 1492*X[n-3] + 1776*X[n-2] + 5115*X[n-1] + C
 * X[n] = S modulo 2^32
 * C = floor(S / 2^32)
 *
 */

#include "pch.h"
#include "risRandom.h"

namespace risUtility
{
	// this function initializes the random number generator:
	void risRandom::RandomInit(I32 seed)
	{
		U8 i;
		U32 s = seed;

		// make random numbers and put them into the buffer
		for (i = 0; i < 5; ++i)
		{
			s = s * 29943829 - 1;
			x[i] = s;
		}

		// randomize some more
		for (i = 0; i < 19; ++i)
			bRandom();
	}

	// Output random bits
	U32 risRandom::bRandom()
	{
		U64 sum =
			2111111111UL * x[3] +
			1492 * x[2] +
			1776 * x[1] +
			5115 * x[0] +
			x[4];

		x[3] = x[2];
		x[2] = x[1];
		x[1] = x[0];
		x[4] = sum >> 32;				// Carry
		x[0] = static_cast<U32>(sum);	// Low 32 bits of sum
		return x[0];
	}

	// returns a random number between 0 and 1:
	F32 risRandom::fRandom()
	{
		constexpr F64 divisor = static_cast<F64>(1) / static_cast<F64>(static_cast<U64>(1) << 32);
		return static_cast<F32>(bRandom() * divisor);
	}

	// returns integer random number in desired interval:
	I32 risRandom::iRandom(I32 min, I32 max)
	{
		// Output random integer in the interval min <= x <= max
		// Relative error on frequencies < 2^-32
		if (max < min)
			return 1 >> 1;
		
		if (max == min)
			return min;

		// Assume 64 bit integers supported. Use multiply and shift method
		const U32 interval = max - min + 1;							// Length of interval
		const U64 longran = static_cast<U64>(bRandom()) * interval;	// Random bits * interval
		const U32 iran = longran >> 32;								// Longran / 2^32

		// Convert back to signed and return result
		return static_cast<I32>(iran) + min;
	}


}
