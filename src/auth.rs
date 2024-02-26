use base64::{prelude::BASE64_STANDARD, Engine};
use hmac::Hmac;
use http::{HeaderMap, HeaderValue};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256, Sha512};
use std::{sync::atomic::{AtomicU64, Ordering}, time::{SystemTime, UNIX_EPOCH}};

use crate::api::endpoint::EndpointType;

#[derive(Debug)]
pub struct Auth {
    api_key: String,
    private_key: Vec<u8>,
    nonce_counter: AtomicU64,
}

impl Auth {
    pub fn new(api_key: String, private_key: String) -> Self {
        Self {
            api_key,
            private_key: BASE64_STANDARD.decode(private_key).unwrap(),
            nonce_counter: AtomicU64::new(0),
        }
    }

    /// Adds the appropriate headers to perform authenticated calls.
    pub fn set_headers(
        &self,
        headers: &mut HeaderMap<HeaderValue>,
        path: &str,
        body: &mut Map<String, Value>,
        endpoint_type: &EndpointType,
    ) {
        match endpoint_type {
            EndpointType::Spot => self.set_headers_spot(headers, path, body),
            EndpointType::Futures => self.set_headers_futures(headers, path, body),
        }
    }

    pub fn set_headers_spot(
        &self,
        headers: &mut HeaderMap<HeaderValue>,
        path: &str,
        body: &mut Map<String, Value>,
    ) {
        let nonce = self.generate_nonce();

        body.insert("nonce".to_string(), Value::Number(nonce.into()));
        let encoded_body = serde_urlencoded::to_string(&body).unwrap();

        let mut sha256 = Sha256::new();
        Sha256::update(&mut sha256, nonce.to_string().as_bytes());
        Sha256::update(&mut sha256, encoded_body.as_bytes());

        let mut hmac_sha512 =
            <Hmac<Sha512> as hmac::Mac>::new_from_slice(&self.private_key[..]).unwrap();
        hmac::digest::Update::update(&mut hmac_sha512, path.as_bytes());
        hmac::digest::Update::update(&mut hmac_sha512, &sha256.finalize());

        let signature: String =
            BASE64_STANDARD.encode(hmac::Mac::finalize(hmac_sha512).into_bytes());

        let api_key_header_value = HeaderValue::from_str(&self.api_key).unwrap();
        let signature_header_value = HeaderValue::from_str(&signature).unwrap();
        headers.insert("API-Key", api_key_header_value);
        headers.insert("API-Sign", signature_header_value);
    }

    fn set_headers_futures(
        &self,
        headers: &mut HeaderMap<HeaderValue>,
        path: &str,
        body: &mut Map<String, Value>,
    ) {
        let nonce = self.generate_nonce();
        let encoded_body = serde_urlencoded::to_string(&body).unwrap();

        let real_path = if path.starts_with("/derivatives") {
            &path[12..]
        } else {
            &path[..]
        };

        let mut sha256 = Sha256::new();
        if !body.is_empty() {
            Sha256::update(&mut sha256, encoded_body.as_bytes());
        }

        Sha256::update(&mut sha256, nonce.to_string().as_bytes());
        Sha256::update(&mut sha256, real_path.to_string().as_bytes());

        let mut hmac_sha512 =
            <Hmac<Sha512> as hmac::Mac>::new_from_slice(&self.private_key[..]).unwrap();
        hmac::digest::Update::update(&mut hmac_sha512, &sha256.finalize());

        let signature: String =
            BASE64_STANDARD.encode(hmac::Mac::finalize(hmac_sha512).into_bytes());

        let api_key_header_value = HeaderValue::from_str(&self.api_key).unwrap();
        let signature_header_value = HeaderValue::from_str(&signature).unwrap();
        let nonce_header_value = HeaderValue::from_str(&nonce.to_string()).unwrap();

        headers.insert("APIKey", api_key_header_value);
        headers.insert("Authent", signature_header_value);
        headers.insert("Nonce", nonce_header_value);
    }

    fn generate_nonce(&self) -> u64 {
        // Use atomic increment for nonce
        let nonce = self.nonce_counter.fetch_add(1, Ordering::SeqCst);

        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let timestamp_nonce = (since_epoch.as_millis() as u64) << 20 | nonce;

        timestamp_nonce
    }
}

// #[cfg(test)]
// mod tests {
//     use base64::{prelude::BASE64_STANDARD, Engine};
//     use hmac::Hmac;
//     use http::{HeaderMap, HeaderValue};
//     use serde_json::{Map, Number, Value};
//     use sha2::{Digest, Sha256, Sha512};
//     use std::time::{SystemTime, UNIX_EPOCH};

//     use crate::auth::Auth;

//     #[test]
//     fn nonce() {
//         let auth = Auth {
//             api_key: "".to_owned(),
//             private_key: vec![]
//         };

//         println!("nonce {:?}", auth.generate_nonce());
//     }

//     #[test]
//     fn it_works() {
//         let nonce: u64 = 1616492376594;

//         let encoded_body =
//             "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25";
//         let mut sha256 = Sha256::new();
//         Sha256::update(&mut sha256, nonce.to_string().as_bytes());
//         Sha256::update(&mut sha256, encoded_body.as_bytes());

//         let key = BASE64_STANDARD.decode("kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==").unwrap();
//         let mut hmac_sha512 = <Hmac<Sha512> as hmac::Mac>::new_from_slice(&key).unwrap();
//         hmac::digest::Update::update(&mut hmac_sha512, "/0/private/AddOrder".as_bytes());
//         hmac::digest::Update::update(&mut hmac_sha512, &sha256.finalize());

//         let signature: String =
//             BASE64_STANDARD.encode(hmac::Mac::finalize(hmac_sha512).into_bytes());
//         println!("signature {:?}", signature);
//     }
// }
