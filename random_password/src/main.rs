fn caesar_cipher_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + shift) % 26) + base) as char
            } else {
                c 
            }
        })
        .collect()
}

fn caesar_cipher_decrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 26 - shift) % 26) + base) as char
            } else {
                c 
            }
        })
        .collect()
}

fn main() {
    let message = "Hello, Amanuel!";
    let shift = 3;

  
    let encrypted = caesar_cipher_encrypt(message, shift);
    println!("Encrypted: {}", encrypted);

  
    let decrypted = caesar_cipher_decrypt(&encrypted, shift);
    println!("Decrypted: {}", decrypted);
}
