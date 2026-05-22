use axum::http::{HeaderMap, HeaderName, HeaderValue};
use sha2::{Digest, Sha256};

pub fn snapshot(h: &HeaderMap) -> Vec<(HeaderName, HeaderValue)> {
    h.iter()
        .filter(|(k, _)| k.as_str().starts_with("x-clewdr-"))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn restore(h: &mut HeaderMap, snap: &[(HeaderName, HeaderValue)]) {
    for (k, v) in snap {
        h.insert(k.clone(), v.clone());
    }
}

pub fn cookie_hash(cookie: &crate::config::ClewdrCookie) -> String {
    let mut h = Sha256::new();
    h.update(cookie.as_str().as_bytes());
    format!("sha256:{}", &hex::encode(h.finalize())[..16])
}
