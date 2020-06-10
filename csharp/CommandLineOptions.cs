using CommandLine;

namespace encryptonize
{
    /// <summary>
    /// Command line options for the secret store CLI.
    /// For all calls, the token (-t, --token) parameter is mandatory.
    /// </summary>
    class Options
    {
        [Option('t', "token", Required = true, HelpText = "The authentication token for the encryption API.")]
        public string Token { get; set; }
    }

    /// <summary>
    /// For the 'put' action, the first argument is both key and secret value
    /// in the form 'key:value'.
    /// Additionally, a group ID can be supplied for secret sharing.
    /// </summary>
    [Verb("put", HelpText = "Put a new value for the given key")]
    class PutOptions : Options
    {
        [Option('g', "group", Required = false, HelpText = "An optional group ID for sharing the secret within a group.")]
        public string Group { get; set; }

        [Value(0, MetaName = "keyAndValue", Required = true, HelpText = "Key and value in the form key:value")]
        public string KeyAndValue
        {
            set
            {
                // Splitting key and value parts for convenience
                var kv = value.Split(":");
                Key = kv[0];
                Value = kv[1];
            }
        }

        public string Key { get; set; }
        public string Value { get; set; }
    }

    /// <summary>
    /// For the 'get' action, the only argument is the key to search for
    /// (besides the authentication token).
    /// </summary>
    [Verb("get", HelpText = "Get the value for the given key")]
    class GetOptions : Options
    {
        [Value(0, MetaName = "key", Required = true, HelpText = "Key for the secret to lookup")]
        public string Key { get; set; }
    }
}

