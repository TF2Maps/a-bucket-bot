use std::io::Cursor;
use openssl::crypto::pkey::PKey;
use rand::Rng;
use rand::os::OsRng;

/// Generates a session key.
pub fn generate_key() -> Vec<u8> {
    let mut session_key = vec![0u8; 32];

    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut session_key);

    session_key
}

/// Encrypts a session key using steam's public key.
pub fn encrypt_key(key: &[u8]) -> Vec<u8> {
    // Load in the key
    let steam_pkey_data = include_bytes!("../assets/steam.pub") as &[u8];
    let steam_pkey = PKey::public_key_from_pem(&mut Cursor::new(steam_pkey_data)).unwrap();

    // Actually perform the encryption
    let encrypted_key = steam_pkey.public_encrypt(key);

    // Return the new key
    encrypted_key
}
