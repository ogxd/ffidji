using System;
using System.Collections.Generic;
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
            Console.WriteLine("-> " + FFIDJI.SampleInterface.SumPair(new FFIDJI.SampleInterface.PairToSum { a = 1, b = 2 }));
            Console.WriteLine("-> " + FFIDJI.SampleInterface.SumArray(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = new int[] { 1, 2 } }));
            Console.WriteLine(string.Join(", ", FFIDJI.SampleInterface.Reverse(new FFIDJI.SampleInterface.ArrayToSum { intsToSum = new int[] { 1, 2, 3 } }).intsToSum));
            Console.ReadKey();
        }
    }
}
