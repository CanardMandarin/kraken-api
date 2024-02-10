use std::time::{SystemTime, UNIX_EPOCH};
use hmac::Hmac;
use http::{HeaderMap, HeaderValue};
use sha2::{Digest, Sha256, Sha512};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

#[derive(Debug)]
pub struct Auth {
    api_key: String,
    private_key: String,
}

impl Auth {
    pub fn new(api_key: String, private_key: String) -> Self {
        Self {
            api_key,
            private_key
        }
    }

    /// Adds the appropriate headers to perform authenticated calls.
    pub fn set_headers(&self, headers: &mut HeaderMap<HeaderValue>, path: &str, body: &[u8]) {
        let nonce = self.generate_nonce();

        // let signature_payload = format!("/0/{path}{nonce}{}", std::str::from_utf8(body).unwrap());
        let nonce_body_hashed = Sha256::digest(nonce.to_string().as_bytes());

        let mut hmac_sha512 = <Hmac::<Sha512> as hmac::Mac>::new_from_slice(self.private_key.as_bytes()).unwrap();
        hmac::digest::Update::update(&mut hmac_sha512, path.as_bytes());
        hmac::digest::Update::update(&mut hmac_sha512, nonce_body_hashed.as_slice());

        let signature = URL_SAFE.encode(hmac::Mac::finalize(hmac_sha512).into_bytes());
        
        let api_key_header_value = HeaderValue::from_str(&self.api_key).unwrap();
        let signature_header_value = HeaderValue::from_str(&signature).unwrap();

        headers.insert("API-Sign", signature_header_value);
        headers.insert("API-Key", api_key_header_value);
    }

    // fn set_headers_spot(&self, headers: &mut HeaderMap<HeaderValue>, path: &str, body: &[u8]) {

    // }

    // fn set_headers_futures(&self, headers: &mut HeaderMap<HeaderValue>, path: &str, body: &[u8]) {
    //     // TODO: implement futures auth
    // }


    fn generate_nonce(&self) -> u64 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let timestamp =
            since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;

        timestamp + 1
    }
}
