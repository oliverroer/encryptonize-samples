using System;
using System.Collections.Generic;
using System.Xml;
using System.Xml.Serialization;
using System.Xml.Schema;
using System.IO;

namespace encryptonize
{
    /// <summary>
    /// SecretStore implements a generic Dictionary (Key-Value-Store) 
    /// and adds XML serialization and helpers for storing and loading the XML files.
    /// For simplicity, this assumes that values can represented as strings,
    /// so any conversion of binary data (base64 encoding) happens outside of this context.
    /// </summary>
    [XmlRoot("Secrets")]
    public class SecretStore<TKey, TValue> : Dictionary<TKey, TValue>, IXmlSerializable
    {
        // Our XML will just look like
        // <Secrets>
        //  <Secret Key="somekey" Value="somevalue"/>
        // </Secrets>
        public XmlSchema GetSchema() { return null; }

        /// <summary>Read key/value pairs from an XML data structure</summary>
        public void ReadXml(XmlReader reader)
        {
            if (reader.IsEmptyElement) { return; }
            reader.Read();

            while (reader.NodeType != XmlNodeType.EndElement)
            {
                object key = reader.GetAttribute("Key");
                object value = reader.GetAttribute("Value");
                this.Add((TKey)key, (TValue)value);
                reader.Read();
            }
        }

        /// <summary>Write key/value pairs to an XML data structure</summary>
        public void WriteXml(XmlWriter writer)
        {
            foreach (var key in this.Keys)
            {
                writer.WriteStartElement("Secret");
                writer.WriteAttributeString("Key", key.ToString());
                writer.WriteAttributeString("Value", this[key].ToString());
                writer.WriteEndElement();
            }
        }

        /// <summary>Save the given SecretStore to a file with the given name</summary>
        public static void Save(SecretStore<TKey, TValue> secrets, string filename)
        {
            // create a suitable serializer for our SecretStore, open a stream to the file
            // and write the data
            XmlSerializer serializer = new XmlSerializer(typeof(SecretStore<TKey, TValue>));
            TextWriter textWriter = new StreamWriter(filename);
            serializer.Serialize(textWriter, secrets);
            textWriter.Close();
        }

        /// <summary>Load a SecretStore from the file with the given name</summary>
        public static SecretStore<TKey, TValue> Load(string filename)
        {
            // create a suitable serializer for our SecretStore, open a stream to the file
            // and read the data
            XmlSerializer serializer = new XmlSerializer(typeof(SecretStore<TKey, TValue>));
            TextReader textReader = new StreamReader(filename);
            var secrets = (SecretStore<TKey, TValue>)serializer.Deserialize(textReader);
            textReader.Close();
            return secrets;
        }
    }
}