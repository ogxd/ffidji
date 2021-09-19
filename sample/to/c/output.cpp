#include "pch.h"
#include "output.h"

int32 Sum(int32 A, int32 B)
{
	return A + B;
}

//int32 SumPair(PairToSum A) {
//	return A.a + A.b;
//}
//
//int32 SumArray(ArrayToSum A) {
//	int32 sum = 0;
//	for (int i = 0; i < A.intsToSum_len; i++)
//	{
//		sum += A.intsToSum_ptr[i];
//	}
//	return sum;
//}
//
//ArrayToSum Reverse(ArrayToSum A)
//{
//	ArrayToSum B;
//	B.intsToSum_ptr = (int32*)malloc(A.intsToSum_len * sizeof(int32));
//	B.intsToSum_len = A.intsToSum_len;
//	for (int i = 0; i < A.intsToSum_len; i++)
//	{
//		B.intsToSum_ptr[i] = A.intsToSum_ptr[A.intsToSum_len - i - 1];
//	}
//	return B;
//}
//
//string ReverseString(string A) {
//	string B;
//	B.utf16_char_ptr = (char16*)malloc(A.utf16_char_len * sizeof(int32));
//	B.utf16_char_len = A.utf16_char_len;
//	for (int i = 0; i < A.utf16_char_len; i++)
//	{
//		B.utf16_char_ptr[i] = A.utf16_char_ptr[A.utf16_char_len - i - 1];
//	}
//	return B;
//}