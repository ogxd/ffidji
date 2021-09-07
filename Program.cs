using System;
using System.IO;
using System.Xml;
using System.Xml.Serialization;

namespace UFFI
{
    class Program
    {
        static void Main(string[] args)
        {
            string filename = args[0];

            Console.WriteLine("Loading " + filename);

            XmlSerializer serializer = new XmlSerializer(typeof(InterfaceDescriptor));

            using Stream reader = new FileStream(filename, FileMode.Open);
            InterfaceDescriptor descriptor = (InterfaceDescriptor)serializer.Deserialize(reader);


            Console.ReadKey();
        }
    }

    public class CsharpInterfaceWriter
    {
        public void Write(InterfaceDescriptor interfaceDescriptor)
        {

        }
    }

    public class CsharpProxyWriter
    {
        public void Write(InterfaceDescriptor interfaceDescriptor)
        {

        }
    }

    [XmlType("Interface")]
    public class InterfaceDescriptor
    {
        [XmlArray(ElementName = "Methods")]
        public Method[] Methods;
    }

    [XmlType("Method")]
    public class Method
    {
        [XmlAttribute(AttributeName = "name")]
        public string Name;

        [XmlArray(ElementName = "Parameters")]
        public Parameter[] Parameters;

        [XmlArray(ElementName = "Returns")]
        public Parameter[] Returns;
    }

    [XmlType("Parameter")]
    public class Parameter
    {
        [XmlAttribute(AttributeName = "name")]
        public string name;

        [XmlAttribute(AttributeName = "type")]
        public string type;
    }
}