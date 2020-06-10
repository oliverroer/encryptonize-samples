<?php

require "vendor/autoload.php";

// Setup Eloquent OR Mapper
use Illuminate\Database\Capsule\Manager as Capsule;

$capsule = new Capsule;
$capsule->setAsGlobal();
$capsule->bootEloquent();

// Helper for nicer output
function info($data)
{
    print_r($data);
    print(PHP_EOL);
}

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();

// Options to set group id and content from command line
$groupId = $argv[1] ?? null;
$data = $argv[2] ?? 'demo-content';

// Creating a new model instance
$m = new \Encryptonize\ExampleModel();

// Setting some public attributes
$m->public_column_1 = "public value 1";
$m->public_column_2 = "public value 2";

info("our model instance with public data only");
info($m->toArray());
info("====");

// Setting some secret values that will be encrypted/decrypted on demand
$m->secret_column_1 = 'secret value 1';
$m->secret_column_2 = 'secret value 2';
info("our model instance with secret values encrypted/decrypted on each access");
info($m->toArray());
info("accessing the secret values");
info($m->secret_column_1);
info($m->secret_column_2);
info("====");

// Setting some secret values that will be encrypted/decrypted only on save or load
$m->secret_column_3 = 'secret value 3';
info("our model instance with secred values encrypted/decrypted on save/load");
info("the model holds the unencrypted data");
info($m->toArray());

// This would need a database connection
// $m->save();
// $m->load();
