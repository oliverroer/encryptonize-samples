<?php

namespace Encryptonize;

trait Encryptonize
{
    /**
    * Returns an ApiClient configured to encrypt and decrypt our data
    * This method could use a global client for the whle application
    * or different setups for each model instance, e.g. 
    * depending on who owns the data, a different auth token might be used
    */
    protected function getEncryptonizeClient()
    {
        // Typically, this would be handled by Laravels System
        // for confg and Service Providers.
        // For simplicity, we just load the credentials from the environment
        // And setup the rest with some defaults
        $config = [
            'url' => $_ENV['ENCRYPTONIZE_BASE_URL'],
            'auth_token' => $_ENV['ENCRYPTONIZE_AUTH_TOKEN'],
            'http_client' => \Buzz\Client\Curl::class,
            'request_factory' => \Nyholm\Psr7\Factory\Psr17Factory::class,
            'stream_factory' => \Nyholm\Psr7\Factory\Psr17Factory::class,
        ];

        return new \Encryptonize\Client(
            new $config['http_client'](new $config['request_factory']()), 
            new $config['request_factory'](),
            new $config['request_factory'](),
            $config['url'],
            $config['auth_token']
        );
    }

    /**
    * This method is called by laravel on every attribute access.
    * All attributes denoted in $encryptonizeOnAccess will be decrypted automatically.
    * Everything else is passed through.
    */
    public function getAttribute($key)
    {
        $value = parent::getAttribute($key);

        if (in_array($key, $this->encryptonizeOnAccess)) {
            $value = $this->getEncryptonizeClient()->decrypt(base64_decode($value));
        }

        return $value;
    }

    /**
    * This method is called by laravel on every attribute assignment.
    * All attributes denoted in $encryptonizeOnAccess will be encrypted automatically.
    * Everything else is passed through.
    */
    public function setAttribute($key, $value)
    {
        if (in_array($key, $this->encryptonizeOnAccess)) {
            $value = base64_encode($this->getEncryptonizeClient()->encrypt($value));
        }

        parent::setAttribute($key, $value);
    }

    /**
    * This method registers handlers for model saving and loading.
    * With these handlers, all attributes denoted in $encryptonizeOnLoad will be encrypted
    * or decrypted automatically on database access.
    * This will only work with event dispatching enabled on eloquent, i.e. a full laravel installation
    */
    protected static function booted()
    {
        static::saving(function ($model) {
            foreach ($model->encryptonizeOnLoad as $attribute) {
                if (isset($model->$attribute)) {
                    $model->$attribute = base64_encode($model->getEncryptonizeClient()->encrypt($model->$attribute));
                }
            }

            parent::saving();
        });

        static::retrieved(function ($model) {
            foreach ($model->encryptonizeOnLoad as $attribute) {
                if (isset($model->$attribute)) {
                    $model->$attribute = $this->getEncryptonizeClient()->decrypt(base64_decode($model->$attribute));
                }
            }

            parent::retrieved();
        });
    }

    /**
    * Since we don't have events on this eloquent installation
    * we can override save/load functionality for models to 
    * achieve encryption and decryption on database access
    */
    public function save(array $options = [])
    {
        foreach ($this->encryptonizeOnLoad as $attribute) {
            if (isset($this->$attribute)) {
                $this->$attribute = base64_encode($this->getEncryptonizeClient()->encrypt($this->$attribute));
            }
        }

        parent::save();
    }

    public function fresh($with = [])
    {
        parent::fresh();
        foreach ($this->encryptonizeOnLoad as $attribute) {
            if (isset($this->$attribute)) {
                $this->$attribute = $this->getEncryptonizeClient()->decrypt(base64_decode($this->$attribute));
            }
        }
    }
}