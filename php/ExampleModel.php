<?php

namespace Encryptonize;

use Illuminate\Database\Eloquent\Model;

class ExampleModel extends Model
{
	// use the Encryptonize trait to do all the work for us
    use Encryptonize;

    // define, which attributes should be encrypted/decrypted on property access
    // model and database will hold the encrypted data
    protected $encryptonizeOnAccess = [
        'secret_column_1',
        'secret_column_2'
    ];

    // definde which attributes should be encrypted/decrypted on save() or load()
    // the model will hold decrypted data, the database will hold encrypted data
    protected $encryptonizeOnLoad = [
        'secret_column_3',
        'secret_column_4'
    ];
}