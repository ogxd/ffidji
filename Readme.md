# FFIDJI

FFIDJI is a Foreign Function Interface code generator.  
You can use it to automatically generate bindings for simple to complex types and delegates between different languages, like calling Rust or C native code from C# for instance.

![crustsharp](https://raw.githubusercontent.com/ogxd/ffidji/master/docs/static/images/ppap.png)

_(It actually works the same way with several different languages, checkout [documentation](https://ogxd.github.io/ffidji/) to see what's supported)_

## Supported Languages

- C# to RUST
- C# to C/C++

_Many more to come!_

## [Full Documentation](https://ogxd.github.io/ffidji/)

## Benchmark

'''
|               Method |          Mean |        Error |       StdDev |     Ratio |  RatioSD |  Gen 0 |  Gen 1 | Allocated |
|--------------------- |--------------:|-------------:|-------------:|----------:|---------:|-------:|-------:|----------:|
|       Concat Managed |      13.90 ns |     0.361 ns |     0.822 ns |      1.00 |     0.00 | 0.0076 |      - |      48 B |
|        Concat FFIDJI |     303.69 ns |     6.162 ns |    10.296 ns |     21.87 |     1.44 | 0.0076 |      - |      48 B |
| Concat Protobuf Grpc | 217,729.81 ns | 4,329.875 ns | 7,807.658 ns | 15,756.08 | 1,126.16 | 1.4648 | 0.4883 |   9,795 B |
'''

## Example

### Interface

```xml
<Interface name="SampleInterface">
    <!--define your custom types-->
    <Type name="PairToSum">
        <Field name="a" type="int32"/>
        <Field name="b" type="int32"/>
    </Type>
    <Type name="ArrayToSum">
        <Field name="intsToSum" type="int32" array="true"/>
    </Type>
    <!--define your methods-->
    <Method name="Sum">
        <Parameter name="A" type="int32"/>
        <Parameter name="B" type="int32"/>
        <Return name="C" type="int32"/>
    </Method>
    <Method name="SumPair">
        <Parameter name="input" type="PairToSum"/>
        <Return name="sum" type="int32"/>
    </Method>
    <Method name="SumArray">
        <Parameter name="input" type="ArrayToSum"/>
        <Return name="sum" type="int32"/>
    </Method>
</Interface>
```

### Command

`ffidji -f csharp MyCsharpProject/MyGeneratedInterface.cs -t c MyCppProject/MyHeader.h -i MyInterface.xml`

### Generated output

#### C#

```csharp
// Autogenerated by FFIDJI

using System;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

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
            int length = size * Marshal.SizeOf<T>();
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
            int length = array.Length * sizeof(T);
            IntPtr ptr = Alloc(length);
            void* u_dst = ptr.ToPointer();
            fixed (T* u_src = &array[0])
            {
                Unsafe.CopyBlock(u_dst, u_src, (uint)length);
            }
            return new Arr<T>(ptr, array.Length);
        }

        [DllImport(LIBRARY_NAME, EntryPoint = "Free_FFI")]
        private static extern void Free(IntPtr ptr);

        [DllImport(LIBRARY_NAME, EntryPoint = "Alloc_FFI")]
        private static extern IntPtr Alloc(int length);

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

        private static unsafe void Free(ArrayToSum_FFI input)
        {
            Free(input.intsToSum.ptr);
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct ArrayToSum_FFI
        {
            public Arr<int32> intsToSum;
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

        [DllImport(LIBRARY_NAME, EntryPoint = "SumPair")]
        private extern static int32 SumPair_FFI(PairToSum input);

        public static int32 SumPair(PairToSum input)
        {
            var input_ffi = Convert(input);
            var result_ffi = SumPair_FFI(input_ffi);
            var result = Convert(result_ffi);
            return result;
        }

        [DllImport(LIBRARY_NAME, EntryPoint = "SumArray")]
        private extern static int32 SumArray_FFI(ArrayToSum_FFI input);

        public static int32 SumArray(ArrayToSum input)
        {
            var input_ffi = Convert(input);
            var result_ffi = SumArray_FFI(input_ffi);
            Free(input_ffi);
            var result = Convert(result_ffi);
            return result;
        }
    }
}
```

#### C/C++

You only need to implement generated interface!

```c++
// Autogenerated by FFIDJI

#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif

typedef int8_t int8;
typedef uint8_t uint8;
typedef int16_t int16;
typedef uint16_t uint16;
typedef int32_t int32;
typedef uint32_t uint32;
typedef long int64;
typedef unsigned long uint64;
typedef float float32;
typedef double float64;

__declspec(dllexport) inline void* Alloc_FFI(int32 length)
{
    return (void*)malloc(length);
}

__declspec(dllexport) inline void Free_FFI(void* ptr)
{
    free(ptr);
}

struct PairToSum
{
    int32 a;
    int32 b;
};

struct ArrayToSum
{
    int32* intsToSum_ptr;
    int intsToSum_len;
};

__declspec(dllexport) int32 Sum(int32 A, int32 B);

__declspec(dllexport) int32 SumPair(PairToSum input);

__declspec(dllexport) int32 SumArray(ArrayToSum input);

#ifdef __cplusplus
}
#endif
```

## Links

See [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/string_arguments/) for more info on how to marshal to rust