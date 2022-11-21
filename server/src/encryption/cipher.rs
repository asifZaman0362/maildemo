use hmac::Hmac;
use sha2::Sha256;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockDecryptMut;
use aes::{
    self,
    cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit},
    Aes256,
};
use cbc;
use pbkdf2::pbkdf2;
use rand::prelude::*;
use rand::rngs::OsRng;

type CbcEncAes256 = cbc::Encryptor<Aes256>;
type CbcDecAes256 = cbc::Decryptor<Aes256>;

fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let mut encrypted_data: Vec<u8> = vec![];
    let iv = OsRng.gen::<[u8; 16]>();
    let cipher = CbcEncAes256::new(GenericArray::from_slice(key), &iv.into());
    for byte in iv {
        encrypted_data.push(byte);
    }
    let filesize = data.len();
    let mut size_bytes = [0u8; 8];
    for i in 0..8 {
        size_bytes[7 - i] = ((filesize & (255 << (i * 8))) >> (8 * i)) as u8;
    }
    for byte in size_bytes {
        encrypted_data.push(byte);
    }
    let mut buff = vec![];
    for byte in data {
        buff.push(*byte);
    }
    let ciphertext = cipher.encrypt_padded_vec_mut::<Pkcs7>(&buff);
    for byte in ciphertext {
        encrypted_data.push(byte);
    }
    encrypted_data
}

fn decrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    assert!(data.len() > 32);
    let mut decrypted_data: Vec<u8> = vec![];
    let iv = &data[..16];
    let cipher = CbcDecAes256::new(GenericArray::from_slice(key), iv.into());
    let size_bytes = &data[16..24];
    let mut filesize = 0usize;
    for i in 0..8 {
        filesize |= (size_bytes[i] as usize) << ((7 - i) * 8);
    }
    let mut buff = vec![];
    for byte in &data[24..] {
        buff.push(*byte);
    }
    let plaintext = cipher.decrypt_padded_vec_mut::<Pkcs7>(&buff).unwrap();
    for byte in plaintext {
        decrypted_data.push(byte);
    }
    decrypted_data
}

fn encrypt_file<P>(
    filename: P,
    target: P,
    pbkdf2_config: pbkdf2::Params,
    password: &str,
) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let params = pbkdf2::Params::default();
    let mut key: [u8; 32] = [0; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &[0; 32], params.rounds, &mut key);
    let data = encrypt(&buf, &key);
    let mut target_file = File::create(target)?;
    target_file.write(&data)?;
    Ok(())
}

fn decrypt_file<P>(filename: P, password: &str) -> std::io::Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    let key = [0u8; 32];
    let data = decrypt(&buf, &key);
    Ok(data)
}
