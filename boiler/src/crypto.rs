use std::io::Cursor;
use openssl::crypto::pkey::PKey;
use openssl::crypto::symm::{Crypter, Mode, Type};
use rand::Rng;
use rand::os::OsRng;

fn generate_data(bytes: usize) -> Vec<u8> {
    let mut session_key = vec![0u8; bytes];

    // TODO: Allow rng to be created only once
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut session_key);

    session_key
}

/// Generates a session key.
pub fn generate_key() -> Vec<u8> {
    generate_data(32)
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

fn crypt_iv(iv: &[u8], key: &[u8], mode: Mode) -> Vec<u8> {
    let crypter = Crypter::new(Type::AES_256_ECB);
    crypter.init(mode, key, "".as_bytes());
    crypter.pad(false);

    // Actually perform the encryption
    let mut buffer = crypter.update(&iv);
    buffer.extend_from_slice(&crypter.finalize());

    buffer
}

fn crypt_data(data: &[u8], key: &[u8], iv: &[u8], mode: Mode) -> Vec<u8> {
    let crypter = Crypter::new(Type::AES_256_CBC);
    crypter.init(mode, key, iv);

    // Actually perform the encryption
    let mut buffer = crypter.update(&data);
    buffer.extend_from_slice(&crypter.finalize());

    buffer
}

pub fn symmetric_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let iv = generate_data(16);

    let mut output = crypt_iv(&iv, key, Mode::Encrypt);
    output.extend_from_slice(&crypt_data(data, key, &iv, Mode::Encrypt));

    output
}

pub fn symmetric_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // Slice out the parts
    let encrypted_iv = &data[0..16];
    let encrypted_data = &data[16..];

    // Perform the decryption
    let iv = crypt_iv(encrypted_iv, key, Mode::Decrypt);
    crypt_data(encrypted_data, key, &iv, Mode::Decrypt)
}
