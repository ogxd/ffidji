using NUnit.Framework;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Test;

internal class Tests
{
    [TestCase("Hello", "World", "HelloWorld")]
    [TestCase("資師展汽現星城", "國然無下們參位", "資師展汽現星城國然無下們參位")]
    [TestCase("😀", "💥", "😀💥")]
    public void Concat(string a, string b, string ab)
    {
        string result = FFIDJI.InterfaceStrings.Concat(a, b);
        Assert.AreEqual(ab, result);
    }

    [TestCase("Hello", "World", "HelloWorld")]
    [TestCase("資師展汽現星城", "國然無下們參位", "資師展汽現星城國然無下們參位")]
    [TestCase("😀", "💥", "😀💥")]
    public void ConcatOptimized(string a, string b, string ab)
    {
        string result = FFIDJI.InterfaceStrings.ConcatOptimized(a, b);
        Assert.AreEqual(ab, result);
    }

    [Test]
    public void ConcatArray()
    {
        string result = FFIDJI.InterfaceStrings.ConcatArray(new FFIDJI.InterfaceStrings.strings { array = new[] { "hello", "dear", "world" }});
        Assert.AreEqual("hellodearworld", result);
    }

    [TestCase("Hello", "World", "HelloWorld")]
    public void ConcatSpeed(string a, string b, string ab)
    {
        string result = null;
        for (int i = 0; i < 1_000_000; i++)
        {
            result = FFIDJI.InterfaceStrings.ConcatOptimized(a, b);
        }
        Assert.AreEqual(ab, result);
    }

    [TestCase("Hello", "World", "HelloWorld")]
    public void ConcatSpeed2(string a, string b, string ab)
    {
        string result = null;
        for (int i = 0; i < 1_000_000; i++)
        {
            result = FFIDJI.InterfaceStrings.Concat(a, b);
        }
        Assert.AreEqual(ab, result);
    }

    [TestCase("Hello", "World", "HelloWorld")]
    public void ConcatSpeedRef(string a, string b, string ab)
    {
        string result = null;
        for (int i = 0; i < 1_000_000; i++)
        {
            result = a + b;
        }
        Assert.AreEqual(ab, result);
    }
}
