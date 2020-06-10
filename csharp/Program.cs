using System;
using CommandLine;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Threading.Tasks;
using System.Text;
using System.IO;

namespace encryptonize
{
    /// <summary>
    /// The Program class encapsulates our main functionality:
    /// 	put key:value
    ///		put key:value --group $GroupId
    ///		get key
    /// </summary>
    class Program
    {
        /// <summary>
        /// API Client for encrypting and decrypting data.
        /// </summary>
        public class Client
        {
            public string Token { get; set; }
            private static readonly HttpClient client = new HttpClient();

            // Base URL of the encryption API
            // Differs for staging, production, ...
            private static readonly string baseUrl = "https://api.encryptonize.cyber-crypt.com/v1/";

            /// <summary>Instantiate a Client with a given authentication token.</summary>
            public Client(string token) => Token = token;

            /// <summary>
            /// Call some endpoint using binary content.
            /// All calls can happen asynchronously and therefore return Task objects.
            /// </summary>
            protected async Task<byte[]> CallBinary(string route, byte[] data)
            {
                // Add authorization to all our requests
                client.DefaultRequestHeaders.Accept.Clear();
                client.DefaultRequestHeaders.Add("Authorization", "ApiToken " + Token);

                // We send binary content, therefore set the type to 'application/octet-stream'
                var content = new ByteArrayContent(data);
                content.Headers.Add("Content-Type", "application/octet-stream");

                try
                {
                    var response = await client.PostAsync(baseUrl + route, content);

                    if (response.StatusCode != HttpStatusCode.OK) {

                        // Everything other than status code 200 is an error
                        // For now we just print the information we get from the API
                        var responseContent = await response.Content.ReadAsStringAsync();
                        Console.WriteLine("API call failed:");
                        Console.Write(responseContent);

                        return null;
                    }

                    var bytes = await response.Content.ReadAsByteArrayAsync();
                    return bytes;
                }
                catch (HttpRequestException exception)
                {
                    // The API emits different error codes and messages
                    // depending on why encryption/decryption has failed
                    // (unknown user, not in group, ...)
                    // Such cases can be handled here.
                    Console.WriteLine("API call failed:");
                    Console.Write(exception.Message);
                }

                return null;
            }

            /// <summary>Encrypt given data using the /enc endpoint.</summary>
            public async Task<byte[]> Encrypt(byte[] data, string groupId)
            {
                // If present, add the group identifier to the call
                var urlParams = "";
                if (groupId != null) { urlParams = "?gid=" + groupId; }

                return await CallBinary("enc" + urlParams, data);
            }

            /// <summary>Decrypt given data using the /dec endpoint.</summary>
            public async Task<byte[]> Decrypt(byte[] data) => await CallBinary("dec", data);
        }

        // The instance of our SecretStore, where we save the encrypted secrets locally
        public static SecretStore<string, string> store = new SecretStore<string, string>();

        /// <summary>
        /// Entrypoint for our CLI Application.
        /// Uses the CommandLine package to parse CLI options
        /// and branches to the correct method (encrypt or decrypt).
        static async Task Main(string[] args)
        {
            // Loading our secret store if it already exists
            try
            {
                store = SecretStore<string, string>.Load(@"secrets.xml");
            }
            catch (System.IO.FileNotFoundException)
            {
                Console.WriteLine("The SecretStore file could not be found and will be created.");
                store = new SecretStore<string, string>();
            }

            // The commandline parser takes care of the arguments
            // and calls the respective method on "enc" or "dec"
            var ret = await Parser.Default.ParseArguments<PutOptions, GetOptions>(args)
                     .MapResult(
                      (PutOptions opts) => Put(opts),
                      (GetOptions opts) => Get(opts),
                      errs => Task.FromResult(1));

            // Saving our secret store to disk as a sharable xml file
            SecretStore<string, string>.Save(store, @"secrets.xml");

        }

        /// <summary>
        /// Put a value into our SecretStore for a certain key.
        /// The value will first be encrypted with the encryption service.
        /// </summary>
        private static async Task<int> Put(PutOptions options)
        {
            // We want the raw binary data for interacting with the API
            // Our secrets in this case are ASCII strings, this depends on your use case
            byte[] bValue = Encoding.ASCII.GetBytes(options.Value);

            Console.WriteLine("Encrypting " + options.Value + " for key " + options.Key);

            // We create the API client and use it to encrypt the binary value (either for ourselves or for any group)
            Client client = new Client(options.Token);
            var cipher = await client.Encrypt(bValue, options.Group);

            // Something went wrong during encryption
            if (cipher == null) {
                return 1;
            }

            // Our SecretStore serializes strings to XML
            // so we encode the binary data to a base64 string first
            string base64Cipher = System.Convert.ToBase64String(cipher);
            store[options.Key] = base64Cipher;

            return 0; // Everything alright
        }

        /// <summary>
        /// Get a value from our SecretStore for a certain key.
        /// The value will be decrypted (if possible) with the service.
        /// </summary>
        private static async Task<int> Get(GetOptions options)
        {
            try
            {
                // We lookup the ciphertext in our secret store
                // Since it is base64 encoded, we have to decode it first
                byte[] bValue = System.Convert.FromBase64String(store[options.Key]);

                Console.WriteLine("Decrypting value for " + options.Key);

                // We create the client and start the decryption process
                Client client = new Client(options.Token);
                var clear = await client.Decrypt(bValue);

                // Something went wrong during encryption
                if (clear == null) {
                    return 1;
                }

                // We got the decrypted value (binary), convert it back to ASCII and show it to the user
                Console.WriteLine(Encoding.ASCII.GetString(clear));
            }
            catch (System.Collections.Generic.KeyNotFoundException)
            {
                Console.WriteLine("No secret for key " + options.Key + " found");
                return 1;
            }

            return 0; // Everything alright
        }
    }
}
