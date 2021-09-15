#include "pch.h"
#include "output.h"

int32 Sum(int32 A, int32 B)
{
	return A + B;
}

int32 SumPair(PairToSum A) {
	return A.a + A.b;
}

int32 SumArray(ArrayToSum A) {
	int32 sum = 0;
	for (int i = 0; i < A.intsToSum_len; i++)
	{
		sum += A.intsToSum_ptr[i];
	}
	return sum;
}