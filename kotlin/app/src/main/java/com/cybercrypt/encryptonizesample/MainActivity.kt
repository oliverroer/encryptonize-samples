package com.cybercrypt.encryptonizesample

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Base64
import android.util.Log
import android.widget.TextView
import com.google.firebase.firestore.Query
import com.google.firebase.firestore.ktx.firestore
import com.google.firebase.ktx.Firebase
import kotlinx.android.synthetic.main.activity_main.*
import java.util.*
import kotlin.collections.ArrayList

class MainActivity : AppCompatActivity() {

    // Setup the firestore configured in firebase. Depends on a valid google-services.json
    val db = Firebase.firestore

    // Setup the ApiClient. Insert your credentials here.
    // Caution: do not ship an app with your credentials included this way.
    // The ApiToken should always be obtained through a separate channel during app setup.
    val apiClient = ApiClient(
        "https://api.encryptonize.cyber-crypt.com/v1",
        "<ApiToken>"
    )

    /**
     * Entry point of our demo app
     */
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Register for Firestore updates
        listenForUpdates()

        // Post a message to the Firestore on button click
        // Since we registered for updates, we should also see the new message automatically
        btnPutNewMessage.setOnClickListener {
            postEncryptedMessage("Fresh Message at " + Date())
        }

        // Optionally we can check for new messages manually
        // messages()
    }

    /**
     * Method to test if we can connect to the API
     */
    private fun testEncryptionService() {
        val testphrase = "test-secret-to-encrypt"
        apiClient.encrypt(testphrase.toByteArray(), null) { response ->
            Log.e(ApiClient.LogTag, "Back from encrypt")
            Log.e(ApiClient.LogTag, String(response))
            apiClient.decrypt(response) { response ->
                Log.e("Encryptonize", String(response))
                assert(String(response).equals(testphrase))
            }
        }
    }

    /**
     * Get all messages from firestore and decrypt, if we can
     */
    private fun messages() {
        db.collection("shared-secrets")
            .get()
            .addOnSuccessListener { result ->
                for (document in result) {
                    Log.d(ApiClient.LogTag, "${document.id} => ${document.data}")
                }
            }
            .addOnFailureListener { exception ->
                Log.w(ApiClient.LogTag, "Error getting documents.", exception)
            }
    }

    /**
     * Register for updated data in firestore
     * On update, show last message in UI
     * and log all messages
     */
    private fun listenForUpdates() {
        db.collection("shared-secrets")
            .orderBy("created_at", Query.Direction.ASCENDING)
            .whereEqualTo("message_type", "demo_type")
            .addSnapshotListener { value, e ->
                if (e != null) {
                    Log.w(ApiClient.LogTag, "Listen failed.", e)
                    return@addSnapshotListener
                }

                val messages = ArrayList<String>()
                for (doc in value!!) {
                    doc.getString("content")?.let {
                        messages.add(it)
                    }
                }

                // Log all current messages
                Log.d(ApiClient.LogTag, "Current messages: $messages")

                // Decrypt and show the last one in our UI
                apiClient.decrypt(Base64.decode(messages.last(), Base64.DEFAULT)) { response ->
                    Log.d(ApiClient.LogTag, "Decrypted: " + String(response))

                    // This has to happen on UI thread again, since it updates the view
                    runOnUiThread {
                        val lastMessage = TextView(this)
                        lastMessage.text = String(response)
                        messageContainer.removeAllViews()
                        messageContainer.addView(lastMessage)
                    }
                }
            }
    }

    /**
     * Encrypt and then post a new message
     */
    private fun postEncryptedMessage(msg: String) {
        apiClient.encrypt(msg.toByteArray(), null) { response ->
            val encryptedMessage = Base64.encodeToString(response, Base64.DEFAULT)
            postMessage(encryptedMessage)
        }
    }

    /**
     * Post new message to firestore
     */
    private fun postMessage(msg: String) {

        val message = hashMapOf(
            "device_id" to CredentialsHelper.getDeviceID(applicationContext),
            "message_type" to "demo_type",
            "content" to msg,
            "created_at" to Date()
        )

        db.collection("shared-secrets")
            .add(message)
            .addOnSuccessListener { documentReference ->
                Log.d(ApiClient.LogTag, "DocumentSnapshot added with ID: ${documentReference.id}")
            }
            .addOnFailureListener { e ->
                Log.w(ApiClient.LogTag, "Error adding document", e)
            }
    }
}
