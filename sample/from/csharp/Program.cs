using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace output
{
    public class Program
    {
        public static void Main(string[] args)
        {
            Console.WriteLine("-> " + FFIDJI.SampleInterface.Sum(1, 2));
            //Console.WriteLine("-> " + FFIDJI.SampleInterface.SumPair(new FFIDJI.SampleInterface.PairToSum { a = 1, b = 2 }));
            //Console.WriteLine("-> " + FFIDJI.SampleInterface.SumArray(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = new int[] { 1, 2 } }));
            //Console.WriteLine(string.Join(", ", FFIDJI.SampleInterface.Reverse(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = new int[] { 1, 2, 3 } }).intsToSum));
            //Console.WriteLine(FFIDJI.SampleInterface.ReverseString("Hello World"));

            //int[] testArray = Enumerable.Range(0, 10).ToArray();

            //var before = Stopwatch.GetTimestamp();
            //for (int i = 0; i < 10_000_000; i++)
            //{
            //    FFIDJI.SampleInterface.Reverse(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = testArray });
            //}
            //Console.WriteLine($"Unmanaged done in {new TimeSpan(Stopwatch.GetTimestamp() - before)}");

            //before = Stopwatch.GetTimestamp();
            //for (int i = 0; i < 10_000_000; i++)
            //{
            //    Reverse(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = testArray });
            //}
            //Console.WriteLine($"Managed done in {new TimeSpan(Stopwatch.GetTimestamp() - before)}");

            Console.WriteLine("done!");

            Console.ReadKey();
        }

        public static FFIDJI.SampleInterface.ArrayToSum Reverse(FFIDJI.SampleInterface.ArrayToSum array)
        {
            var result = new FFIDJI.SampleInterface.ArrayToSum();
            result.intsToSum = new int[array.intsToSum.Length];
            for (int i = 0; i < array.intsToSum.Length; i++)
            {
                result.intsToSum[i] = array.intsToSum[array.intsToSum.Length - i - 1];
            }
            return result;
        }
    }
}
