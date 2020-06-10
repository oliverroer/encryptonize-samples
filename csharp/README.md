# .NET Core (C#) Example Using the Encryptonize API

This example implements a secret store. Secrets are saved within a file locally.
Since all secrets are encrypted, a single file could be used for all users and shared, using e.g. a cloud drive.
Only users assigned to correct group will be able to decrypt the secrets.

You can obtain the respective API token, user and group IDs from the [Encryptonize Frontend](https://encryptonize.cyber-crypt.com).

## Build and run

This assumes you have [.NET Core](https://dotnet.microsoft.com/download) and its tooling installed.

To build & run from the command line, navigate into the project folder and issue the following

```bash
# put secrets into the secretstore
dotnet run put somekey:secretvalue --token <ApiToken>

# store secrets for a group of users
dotnet run put somekey:secretvalue --token <ApiToken>  --group <GroupId>

# get secrets
dotnet run get somekey --token <ApiToken>
```