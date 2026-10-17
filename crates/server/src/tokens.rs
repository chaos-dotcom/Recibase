//! Opaque random tokens for sessions, login `state`/`nonce` and CSRF.

use getrandom::getrandom;
use sha2::{Digest, Sha256};

const HEX: &[u8; 16] = b"0123456789abcdef";

/// `bytes` random bytes from the OS source, lower-case hex encoded.
///
/// Used for values that must be unguessable: session ids, OAuth `state` and
/// `nonce`, and CSRF tokens.
pub fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    getrandom(&mut buf).expect("the OS random source is available");
    let mut out = String::with_capacity(bytes * 2);
    for byte in buf {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

/// An RFC 7636 PKCE pair: a high-entropy `code_verifier` and its `S256`
/// `code_challenge` (base64url of the SHA-256 digest, no padding).
pub fn pkce_pair() -> (String, String) {
    let verifier = random_hex(32);
    let digest = Sha256::digest(verifier.as_bytes());
    (verifier, base64_url_no_pad(&digest))
}

/// Base64url without padding (RFC 4648 §5), the encoding PKCE and JWTs use.
fn base64_url_no_pad(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let first = chunk[0] as u32;
        let second = *chunk.get(1).unwrap_or(&0) as u32;
        let third = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (first << 16) | (second << 8) | third;
        out.push(ALPHABET[((triple >> 18) & 0x3f) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((triple >> 6) & 0x3f) as usize] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(triple & 0x3f) as usize] as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{base64_url_no_pad, pkce_pair, random_hex};

    #[test]
    fn a_token_is_two_hex_chars_per_byte() {
        assert_eq!(random_hex(32).len(), 64);
        assert!(random_hex(16).bytes().all(|b| b.is_ascii_hexdigit()));
    }

    #[test]
    fn tokens_differ_between_calls() {
        assert_ne!(random_hex(32), random_hex(32));
    }

    #[test]
    fn base64url_matches_known_vectors() {
        assert_eq!(base64_url_no_pad(&[0xfb, 0xff]), "-_8");
        // RFC 4648 test vector, unpadded.
        assert_eq!(base64_url_no_pad(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn a_pkce_pair_is_well_formed_and_unique() {
        let (verifier, challenge) = pkce_pair();
        assert_eq!(verifier.len(), 64, "a 32-byte verifier in hex");
        assert_eq!(challenge.len(), 43, "an unpadded SHA-256 digest in base64url");
        assert!(
            challenge
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
            "the challenge is base64url"
        );
        assert_ne!(pkce_pair().0, verifier);
    }
}
