use aes::{
    cipher::{
        BlockCipher, BlockEncrypt, 
        BlockDecrypt, KeyInit, KeyIvInit,
        block_padding::Pkcs7, generic_array::GenericArray
    },
    Aes256, Aes256Enc, Aes256Dec
};
use rand::prelude::*;
use rand::rngs::OsRng;


type Cbc256enc = cbc::Encryptor<Aes256>;
type Cbc256dec = cbc::Decryptor<Aes256>;

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let encrypted_data: Vec<u8> = vec![];
    let iv = OsRng.gen::<[u8; 16]>();
    let mut _buffer: Vec<u8> = vec![];
    let _enc = Cbc256enc::new(GenericArray::from_slice(key), &iv.into());
    encrypted_data
}
