package com.cybercrypt.encryptonizesample

import android.util.Log
import okhttp3.*
import okio.BufferedSink
import okhttp3.MediaType
import okhttp3.MediaType.Companion.toMediaTypeOrNull
import java.io.IOException

/**
 * Custom RequestBody to make sure our raw data is processed correctly
 * and the correct content-type header is set
 */
class ByteArrayRequestBody(val data: ByteArray) : RequestBody() {
    override fun contentType(): MediaType? {
        return "application/octet-stream".toMediaTypeOrNull()
    }

    override fun writeTo(sink: BufferedSink) {
        sink.write(data)
    }
}

/**
 * ApiClient class to wrap calls to the encryption API
 */
class ApiClient(val baseUrl: String, val authToken: String) {

    companion object {
        // We'll use this tag for logging throughout the demo application
        const val LogTag = "Encryptonize"
    }

    // OkHttpClient will make our HTTP calls
    private val client = OkHttpClient()

    /**
     * Method to call /enc or /dec routes with raw data as content
     * On success, the callback function will be executed
     */
    fun callBinary(route: String, data: ByteArray, callback: (response: ByteArray) -> Unit) {

        // Every request will need to be authorized
        val request = Request.Builder()
            .url(baseUrl + "/" + route)
            .addHeader("Authorization", "ApiToken " + authToken)
            .post(ByteArrayRequestBody(data))
            .build()

        // Enqueue the request
        // Network calls on the main thread are not allowed in android
        // So this has to be aync
        client.newCall(request).enqueue(object : Callback {
            override fun onFailure(call: Call, e: IOException) {
                Log.e(ApiClient.LogTag, e.message)
            }

            override fun onResponse(call: Call, response: Response) {
                response.use {
                    if (!response.isSuccessful) {
                        // Anything other than response codes indicating success means that
                        // there was a problem with our request
                        Log.d(ApiClient.LogTag, "$response")
                    } else {
                        // API call Successful, execute callback
                        Log.e(ApiClient.LogTag, "received API response")
                        callback(response.body!!.bytes())
                    }
                }
            }
        })
    }

    /**
     * Simplified call to /enc
     */
    fun encrypt(data: ByteArray, group: String?, callback: (response: ByteArray) -> Unit) {
        // Add group parameter to URL if supplied
        val params = if (group != null) ("?gid=" + group!!) else ""
        Log.d(ApiClient.LogTag, "Encrypting")
        callBinary("enc" + params, data, callback)
    }

    /**
     * Simplified call to /dec
     */
    fun decrypt(data: ByteArray, callback: (response: ByteArray) -> Unit) {
        Log.d(ApiClient.LogTag, "Decrypting")
        callBinary("dec", data, callback)
    }
}