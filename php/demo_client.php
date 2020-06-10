<?php

require 'vendor/autoload.php';

// helper for nicer output
function info($data)
{
    print_r($data);
    print(PHP_EOL);
}

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();

// Options to set group id and content from command line
$data = $argv[1] ?? 'demo-content';
$groupId = $argv[2] ?? null;

info('setting up api client');
$requestFactory = new \Nyholm\Psr7\Factory\Psr17Factory();

// api client setup
// the client should work with any PSR18 compliant HTTP client (and corresponding PSR7/PSR17 factories)
$client = new \Encryptonize\Client(
    new \Buzz\Client\Curl($requestFactory), // any compliant http client, e.g. new Http\Adapter\Guzzle6\Client(),
    $requestFactory, // request factory
    $requestFactory, // stream factory
    $_ENV['ENCRYPTONIZE_BASE_URL'], // api base url
    $_ENV['ENCRYPTONIZE_AUTH_TOKEN']); // authentication token

info(PHP_EOL . 'encrypting');

$cipher = $client->encrypt($data, $groupId);

info('=====');
info(bin2hex($cipher));
info(PHP_EOL . 'decrypting');

$clear = $client->decrypt($cipher);

info('=====');
info($clear);

assert($clear === $data);
info('all fine, done');