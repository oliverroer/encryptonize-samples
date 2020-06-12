//! Encryption and decryption via the Encryptonize service.
use reqwest;
use reqwest::Url;

/// Represents access to the Encryptonize service for a particular user.
pub struct Encryptonize {
    base_url: Url,
    user_token: String,
    group: Option<String>,
}

impl Encryptonize {
    const BASE_URL: &'static str = "http://127.0.0.1:8000/v1/";

    /// Create access to Encryptonize for a single user.
    ///
    /// # Arguments
    /// * `user_token` - Encryptonize user token.
    /// * `group` - Optional Group ID. Members of this group can decrypt all data encrypted with
    ///   this instance.
    pub fn new(user_token: &str, group: Option<&str>) -> Encryptonize {
        Encryptonize {
            base_url: Url::parse(Encryptonize::BASE_URL).unwrap(),
            user_token: user_token.to_string(),
            group: group.map(|x| x.to_string()),
        }
    }

    /// Encrypt data for the user and group (if specified).
    ///
    /// # Arguments
    /// * `data` - A string to encrypt.
    pub fn encrypt(&self, data: String) -> Result<Vec<u8>, String> {
        let group = match &self.group {
            Some(group) => format!("?gid={}", group),
            None => String::new(),
        };

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(self.base_url.join(&format!("enc{}", group)).unwrap())
            .header("Content-Type", "application/octet-stream")
            .header("Authorization", format!("ApiToken {}", self.user_token))
            .body(data)
            .send()
            .map_err(|_| format!("Encryption failed"))?;

        if !response.status().is_success() {
            return Err(format!(
                "Encryption failed with status {:?}",
                response.status()
            ));
        }

        return response
            .bytes()
            .map(|x| Vec::from(x.as_ref()))
            .map_err(|_| "Error decoding body".to_string());
    }

    /// Decrypt data. Will return an `Err` if the user is not allowed to decrypt.
    ///
    /// # Arguments
    /// * `data` - A vector of bytes to decrypt.
    pub fn decrypt(&self, data: Vec<u8>) -> Result<String, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(self.base_url.join("dec").unwrap())
            .header("Content-Type", "application/octet-stream")
            .header("Authorization", format!("ApiToken {}", self.user_token))
            .body(data)
            .send()
            .map_err(|_| format!("Decryption failed"))?;

        if !response.status().is_success() {
            return Err(format!(
                "Decryption failed with status {:?}",
                response.status()
            ));
        }

        return response
            .text()
            .map_err(|_| "Error decoding body".to_string());
    }
}
