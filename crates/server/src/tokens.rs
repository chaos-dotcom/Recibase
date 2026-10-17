//! Opaque random tokens for sessions, login `state`/`nonce` and CSRF.

use getrandom::getrandom;

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

#[cfg(test)]
mod tests {
    use super::random_hex;

    #[test]
    fn a_token_is_two_hex_chars_per_byte() {
        assert_eq!(random_hex(32).len(), 64);
        assert!(random_hex(16).bytes().all(|b| b.is_ascii_hexdigit()));
    }

    #[test]
    fn tokens_differ_between_calls() {
        assert_ne!(random_hex(32), random_hex(32));
    }
}
