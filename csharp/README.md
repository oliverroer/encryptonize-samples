# Dotnet Core (C#) Example Using The Encryption API

This example implements a secret store. Secrets are saved within a file locally.
Since all secrets are encrypted, a single file could be used for all users and shared, using e.g. a cloud drive.
Only users assigned to correct group will be able to decrypt the secrets.

You can obtain the respective API token, user and group IDs from the encryptonize frontend.

## Build and run

This assumes you have dotnet core and its tooling installed.

To build & run from the command line, navigate into the project folder and issue the following

´´´
# build (creates an encryptonize.dll in bin/Debug/netcoreapp3.1)
dotnet build

# running the created dll
# put secrets into the secretstore
dotnet encryptonize.dll put somekey:secretvalue --token <ApiToken>

# store secrets for a group of users
dotnet encryptonize.dll put somekey:secretvalue --token <ApiToken> --group <GroupId>

# get secrets
dotnet encryptonize.dll get somekey --token <ApiToken>


# alternatively you can use 'dotnet run' from within the project folder

# run - encrypt $file for user $token
dotnet run put somekey:secretvalue --token <ApiToken>

# run - decrypt $file for user $token
dotnet run get somekey --token <ApiToken>

´´´