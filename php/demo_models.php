<?php

require 'vendor/autoload.php';
use Illuminate\Database\Capsule\Manager as Capsule;

// Helper for nicer output
function info($data)
{
    print_r($data);
    print(PHP_EOL);
}

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();


// create sqlite database file if it doesn't exist yet
if (!file_exists($_ENV['DB_DATABASE'])) {
	touch($_ENV['DB_DATABASE']);
}

// Setup Eloquent OR Mapper
$capsule = new Capsule;

$capsule->addConnection([
    "driver" => "sqlite",
    "database" => $_ENV['DB_DATABASE'],
]);

$capsule->setAsGlobal();
$capsule->bootEloquent();

if (!Capsule::schema()->hasTable('secrets')) {
    Capsule::schema()->create('secrets', function ($table) {
            $table->increments('id');
            $table->string('public_column_1')->nullable();
            $table->string('public_column_2')->nullable();
            $table->string('secret_column_1')->nullable();
            $table->string('secret_column_2')->nullable();
            $table->string('secret_column_3')->nullable();
            $table->string('secret_column_4')->nullable();
            $table->timestamps();
    });
}

// Creating a new model instance
$m = new \Encryptonize\ExampleModel();

// Setting some public attributes
$m->public_column_1 = 'public value 1';
$m->public_column_2 = 'public value 2';

info('our model instance with public data only');
info($m->toArray());
info('====');

// Setting some secret values that will be encrypted/decrypted on demand
$m->secret_column_1 = 'secret value 1';
$m->secret_column_2 = 'secret value 2';
info('our model instance with secret values encrypted/decrypted on each access');
info($m->toArray());
info('accessing the secret values');
info($m->secret_column_1);
info($m->secret_column_2);
info('====');

// Setting some secret values that will be encrypted/decrypted only on save or load
$m->secret_column_3 = 'secret value 3';
$m->secret_column_4 = 'secret value 4';
info('our model instance with secret values encrypted/decrypted on save/load');
info('the model holds the unencrypted data');
info($m->toArray());

// Saving to database, updating the model
$m->save();
$m->fresh();

info("model stored with id " . $m->id . ", holds unencrypted data for secret values 3 and 4");
info($m->toArray());

info("selecting data directly from database, secret values 3 and 4 are encrypted");
$queryResult = Capsule::table('secrets')->where('id', $m->id)->get();
info($queryResult);