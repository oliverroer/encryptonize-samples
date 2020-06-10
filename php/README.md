# Encryptonize API PHP Examples

The `demo_client.php` example shows, how you can setup the provided client to interact with the Encryptonize API.

The `demo_models.php` example shows, how you can integrate this client with Laravel's Eloquent OR Mapper for transparent encryption and decryption on your model instances.

## Features

`demo_models.php` will create a sqlite database according to the database name set in `.env`.
The demo creates a model instance and proceeds to set public and secret attributes, as well as storing them to the database. The output shows at which point the data is encrypted.

## Contents

`Client.php` - Wrapper for the API calls, can use any PSR18 HTTP Client (https://www.php-fig.org/psr/psr-18/)
`Encryptonize.php` - Trait to be used on Eloquent Models to add Encryptonize functionality
`ExampleModel.php` - Eloquent Model using the `Encryptonize` Trait
`demo_client.php` - Example usage for the `Client`
`demo_models.php` - Example usage for Eloquent Models using the `Encryptonize` Trait

## Build and run

This example requires PHP >= 7.2 and Composer (https://getcomposer.org/) to load dependencies.

* add a `.env` file according to `.env.example`.
* run `composer install`

*To run the client example*

```
php demo_client.php

# or

php demo_client.php <content_to_encrypt> <group_id>
```

*To run the Eloquent demo*

```
php demo_models.php
```
