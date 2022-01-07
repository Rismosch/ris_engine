/*
 * Mother-of-All Pseudorandom Number Generators, adapted for my purposes.
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

#pragma once
#include "../risData/risData.h"

namespace risUtility
{
	using namespace risData;

	class risRandom
	{
	public:
		void RandomInit(I32 seed);		// Initialization
		U32 bRandom();					// Output random bits
		F32 fRandom();					// Get floating point random number
		I32 iRandom(I32 min, I32 max);	// Get integer random number in desired interval

		risRandom(I32 seed) : x(new U32[5]) { RandomInit(seed); }
		~risRandom() { delete[] x; }
	protected:
		U32* x;							// History buffer
	};
}
