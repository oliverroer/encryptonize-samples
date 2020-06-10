<?php

namespace Encryptonize;

use Psr\Http\Client\ClientInterface;
use Psr\Http\Client\ClientExceptionInterface;
use Psr\Http\Message\RequestFactoryInterface;
use Psr\Http\Message\StreamFactoryInterface;

/**
* Client for the Encryptonize API
* This client can use any PSR18 (https://www.php-fig.org/psr/psr-18/) conform HTTP client
* Thus, it is necessary to instantiate it with the chosen client library 
* and factories for request creation and stream handling
*/
class Client
{
    protected $baseUrl;
    protected $authToken;
    protected $httpClient;
    protected $httpRequestFactory;
    protected $streamFactory;

    public function __construct(ClientInterface $httpClient, RequestFactoryInterface $httpRequestFactory, StreamFactoryInterface $streamFactory, string $baseUrl, string $authToken)
    {
        $this->httpClient = $httpClient;
        $this->httpRequestFactory = $httpRequestFactory;
        $this->streamFactory = $streamFactory;
        $this->baseUrl = $baseUrl;
        $this->authToken = $authToken;
    }

    /**
    * Build a request to /enc or /dec
    * with the given data
    * PHP Strings also function as Byte Arrays, so no conversion is necessary
    */
    protected function callBinary(string $route, $data)
    {
        $url = $this->baseUrl . '/' . $route;
        $request = $this->httpRequestFactory->createRequest('POST', $url)
            ->withHeader('Authorization', 'ApiToken ' . $this->authToken)
            ->withHeader('Content-Type', 'application/octet-stream')
            ->withBody($this->streamFactory->createStream($data));

        try {
            $response = $this->httpClient->sendRequest($request);
        } catch (ClientExceptionInterface $e) {
            echo($e->getMessage());
        }

        return $response->getBody()->getContents() ?? null;
    }

    /**
    * Encrypt the given data
    * optionally for the given groupId
    */
    public function encrypt($data, string $groupId = null)
    {
        $params = is_null($groupId) ? '' : ('?gid=' . $groupId);
        return $this->callBinary('enc' . $params, $data);
    }
    
    /**
    * Decrypt the given ciphertext
    */
    public function decrypt($data)
    {
        return $this->callBinary('dec', $data);
    }
}