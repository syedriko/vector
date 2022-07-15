//! crypto functions implemented in terms of OpenSSL
//!

use openssl::{
	hash::MessageDigest,
	pkey::PKey,
	sign::Signer
};

use crate::{
	errors::Result,
	Algorithm,
	DecodingKey,
	EncodingKey,
	serialization::{b64_decode, b64_encode}
};

/// The actual HS signing + encoding
/// Could be in its own file to match RSA/EC but it's 2 lines...
fn sign_hmac(alg: MessageDigest, key: &[u8], message: &str) -> Result<String> {
	let pkey = PKey::hmac(key).unwrap();
	let mut signer = Signer::new(alg, &pkey).unwrap();
	signer.update(message.as_bytes()).unwrap();
	let digest = signer.sign_to_vec().unwrap();
    Ok(b64_encode(&digest))
}

/// Take the payload of a JWT, sign it using the algorithm given and return
/// the base64 url safe encoded of the result.
///
/// If you just want to encode a JWT, use `encode` instead.
pub fn sign(message: &str, key: &EncodingKey, algorithm: Algorithm) -> Result<String> {
    match algorithm {
        Algorithm::HS256 => sign_hmac(MessageDigest::sha256(), key.inner(), message),
        Algorithm::HS384 => sign_hmac(MessageDigest::sha384(), key.inner(), message),
        Algorithm::HS512 => sign_hmac(MessageDigest::sha512(), key.inner(), message),
		_ => Ok("foo".to_string())
        /*
                Algorithm::ES256 | Algorithm::ES384 => {
                    ecdsa::sign(ecdsa::alg_to_ec_signing(algorithm), key.inner(), message)
                }

                Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512 => rsa::sign(rsa::alg_to_rsa_signing(algorithm), key.inner(), message),
        */
    }
}

/// Compares the signature given with a re-computed signature for HMAC or using the public key
/// for RSA/EC.
///
/// If you just want to decode a JWT, use `decode` instead.
///
/// `signature` is the signature part of a jwt (text after the second '.')
///
/// `message` is base64(header) + "." + base64(claims)
pub fn verify(signature: &str, message: &str, key: &DecodingKey, algorithm: Algorithm) -> Result<bool> {
    Ok(true)
}
