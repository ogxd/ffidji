// Autogenerated by FFIDJI

using System;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;
using System.Security;

using char16 = System.Char;
using int8 = System.SByte;
using uint8 = System.Byte;
using int16 = System.Int16;
using uint16 = System.UInt16;
using int32 = System.Int32;
using uint32 = System.UInt32;
using int64 = System.Int64;
using uint64 = System.UInt64;
using float16 = System.Half;
using float32 = System.Single;
using float64 = System.Double;

namespace FFIDJI
{ 
    public static class SampleInterface
    { 
        public const string LIBRARY_NAME = "MyNativeLibrary.dll";

        private readonly struct Arr<T>
        { 
            public readonly IntPtr ptr;
            public readonly int size;
            public Arr(IntPtr ptr, int size)
            { 
                this.ptr = ptr;
                this.size = size;
            } 
        } 

        private unsafe static T[] CopyArray<T>(IntPtr ptr, int size) where T : unmanaged
        { 
            int length = size * sizeof(T);
            T[] array = new T[size];
            void* u_src = ptr.ToPointer();
            fixed (T* u_dst = &array[0])
            { 
                Unsafe.CopyBlock(u_dst, u_src, (uint)length);
            } 
            return array;
        } 

        private static T[] Convert<T>(Arr<T> arr) where T : unmanaged
        { 
            return CopyArray<T>(arr.ptr, arr.size);
        } 

        private static T Convert<T>(T obj) where T : unmanaged
        { 
            return obj;
        } 

        private unsafe static Arr<T> Convert<T>(T[] array) where T : unmanaged
        { 
            return Convert(new ReadOnlySpan<T>(array));
        } 

        private unsafe static Arr<T> Convert<T>(ReadOnlySpan<T> array) where T : unmanaged
        { 
            int length = array.Length * sizeof(T);
            IntPtr ptr = Alloc(length);
            void* u_dst = ptr.ToPointer();
            fixed (T* u_src = &array[0])
            { 
                Unsafe.CopyBlock(u_dst, u_src, (uint)length);
            } 
            return new Arr<T>(ptr, array.Length);
        } 

        [SuppressUnmanagedCodeSecurity]
        [DllImport(LIBRARY_NAME, EntryPoint = "Alloc_FFI")]
        private static extern IntPtr Alloc(int length);

        [SuppressUnmanagedCodeSecurity]
        [DllImport(LIBRARY_NAME, EntryPoint = "Free_FFI")]
        private static extern void Free(IntPtr ptr, int length);

        [StructLayout(LayoutKind.Sequential)]
        public struct PairToSum
        { 
            public int32 a;
            public int32 b;
        } 

        [StructLayout(LayoutKind.Sequential)]
        public struct ArrayToSum
        { 
            public int32[] intsToSum;
        } 

        [StructLayout(LayoutKind.Sequential)]
        private struct ArrayToSum_FFI
        { 
            public Arr<int32> intsToSum;
        } 

        private static unsafe void Free(ArrayToSum_FFI input)
        { 
            Free(input.intsToSum.ptr, input.intsToSum.size * Marshal.SizeOf<int32>());
        } 

        private static ArrayToSum Convert(ArrayToSum_FFI data_FFI)
        { 
            return new ArrayToSum
            { 
                intsToSum = Convert(data_FFI.intsToSum),
            };
        } 

        private static ArrayToSum_FFI Convert(ArrayToSum data)
        { 
            return new ArrayToSum_FFI
            { 
                intsToSum = Convert(data.intsToSum),
            };
        } 

        private unsafe static ArrayToSum[] Convert(Arr<ArrayToSum_FFI> arr)
        { 
            var array_ffi = CopyArray<ArrayToSum_FFI>(arr.ptr, arr.size);
            var array = new ArrayToSum[arr.size];
            for (int i = 0; i < arr.size; ++i) array[i] = Convert(array_ffi[i]);
            return array;
        } 

        [StructLayout(LayoutKind.Sequential)]
        private struct string_FFI
        { 
            public Arr<char16> utf16_char;
        } 

        private static unsafe void Free(string_FFI input)
        { 
            Free(input.utf16_char.ptr, input.utf16_char.size * Marshal.SizeOf<char16>());
        } 

        private static string Convert(string_FFI data_FFI)
        { 
            unsafe
            { 
                return new string((char*)data_FFI.utf16_char.ptr);
            } 
        } 

        private static string_FFI Convert(string data)
        { 
            return new string_FFI
            { 
                utf16_char = Convert(data.AsSpan())
            };
        } 

        private unsafe static string[] Convert(Arr<string_FFI> arr)
        { 
            var array_ffi = CopyArray<string_FFI>(arr.ptr, arr.size);
            var array = new string[arr.size];
            for (int i = 0; i < arr.size; ++i) array[i] = Convert(array_ffi[i]);
            return array;
        } 

        [SuppressUnmanagedCodeSecurity]
        [DllImport(LIBRARY_NAME, EntryPoint = "Sum")]
        private extern static int32 Sum_FFI(int32 A, int32 B);

        public static int32 Sum(int32 A, int32 B)
        { 
            var A_ffi = Convert(A);
            var B_ffi = Convert(B);
            var result_ffi = Sum_FFI(A_ffi, B_ffi);
            var result = Convert(result_ffi);
            return result;
        } 

        [SuppressUnmanagedCodeSecurity]
        [DllImport(LIBRARY_NAME, EntryPoint = "Reverse")]
        private extern static ArrayToSum_FFI Reverse_FFI(ArrayToSum_FFI input);

        public static ArrayToSum Reverse(ArrayToSum input)
        { 
            var input_ffi = Convert(input);
            var result_ffi = Reverse_FFI(input_ffi);
            Free(input_ffi);
            var result = Convert(result_ffi);
            Free(result_ffi);
            return result;
        } 
    } 
} 
