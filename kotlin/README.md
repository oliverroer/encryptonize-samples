# Encryptonize Kotlin Example

This example Android app uses Google Firebase Firestore to exchange and store data that should be protected, and the encryption API to do so.

## Features

The app includes a basic UI. By clicking "PUT NEW MESSAGE", a new message will be created (simply including the current date and time),
encrypted and stored in Firestore. Any new message in Firestore (with a specific type) will be displayed below the button.

The data structure in Firestore is as follows

```
{
    device_id: String (UUID of the device so we might determine the sender)
    message_type: String (for subscribing to messages of a given type)
    content: String (Base64 encoded, encrypted message content)
    created_at: Date (for ordering messages for our purposes)
}
```

Our firestore collection is called `shared-secrets`, and we listen for messages of type `demo_type`.

## Building and Running

To run the app, you need to drop your own `google-services.json` into the `/app` folder.
For setting up a Firestore collection, please see https://firebase.google.com/docs/firestore/quickstart.

Additionally, you need to configure your own ApiToken in MainActivity.kt.

With these preconditions fulfilled, you can build and run the app in Android Studio.


