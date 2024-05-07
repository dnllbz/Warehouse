use magic_crypt::{MagicCryptTrait, new_magic_crypt};

pub fn crypt(string: &str, key: &str) -> String {
    new_magic_crypt!(key, 256).encrypt_str_to_base64(string)
}

pub fn decrypt(string: &str, key: &str) -> String {
    new_magic_crypt!(key, 256).decrypt_base64_to_string(string).unwrap_or_else(|_| {
        println!("Something went wrong while decrypting... (possibly: bad key)");
        String::new()
    })
}