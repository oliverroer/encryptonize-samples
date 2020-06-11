use reqwest;
use reqwest::Url;

pub struct Encryptonize {
    base_url: Url,
    user_token: String,
}

impl Encryptonize {
    const BASE_URL: &'static str = "http://127.0.0.1:8000/v1/";

    pub fn new(user_token: &str) -> Encryptonize {
        Encryptonize {
            base_url: Url::parse(Encryptonize::BASE_URL).unwrap(),
            user_token: user_token.to_string(),
        }
    }

    pub fn encrypt(&self, data: String) -> Result<Vec<u8>, String> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(self.base_url.join("enc").unwrap())
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
