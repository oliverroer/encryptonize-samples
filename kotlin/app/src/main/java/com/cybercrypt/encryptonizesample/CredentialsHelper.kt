package com.cybercrypt.encryptonizesample

import android.content.Context
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKeys
import java.io.IOException
import java.security.GeneralSecurityException
import java.util.*

/**
 * Helper singleton to create, save and load a unique identifier for our device
 * so that we can send this with our messages and see where they come from
 * or target specific devices
 */
object CredentialsHelper {
    /**
     * We use Encrypted Shared Preferences to store our device ID
     * see https://developer.android.com/topic/security/data for reference
     */
    @Throws(GeneralSecurityException::class, IOException::class)
    private fun getEncryptedSharedPreferences(context: Context): SharedPreferences {
        val masterKeyAlias = MasterKeys.getOrCreate(MasterKeys.AES256_GCM_SPEC)
        return EncryptedSharedPreferences.create(
            "credentials",
            masterKeyAlias,
            context,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
        )
    }

    /**
     * Clearing the credentials allows us to reset the app
     * and generate new ones
     */
    @Throws(GeneralSecurityException::class, IOException::class)
    fun clearCredentials(context: Context) {
        val sharedPreferences =
            getEncryptedSharedPreferences(context)
        sharedPreferences.edit().clear().commit()
    }

    /**
     * Get the device ID from shared preferences
     * If not present, generate a new one
     */
    @Throws(GeneralSecurityException::class, IOException::class)
    fun getDeviceID(context: Context): String? {
        val sharedPreferences =
            getEncryptedSharedPreferences(context)
        if (!sharedPreferences.contains("device_id")) {
            sharedPreferences.edit()
                .putString("device_id", UUID.randomUUID().toString())
                .commit()
        }
        return sharedPreferences.getString("device_id", null)
    }
}