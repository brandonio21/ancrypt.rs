pub trait Cipher {
    fn encrypt(&self, plaintext: String) -> Result<String, String>;
    fn decrypt(&self, encrypted_text: String) -> Result<String, String>;
    fn bash(&self, encrypted_text: String) -> Result<String, String>;
}

pub mod vigenere {
    use std::collections::HashMap;
    use super::Cipher;

    pub struct VigenereCipher {
        pub key: String,
        pub lookup_table: HashMap<usize, HashMap<char, char>>,
    }

    impl VigenereCipher {

        /// Return a new VigenereCipher with a blank key by default
        pub fn new(key_str: &str) -> VigenereCipher {
            let mut new_vigenere_cipher = VigenereCipher {
                key: key_str.to_string(),
                lookup_table: HashMap::new(),
            };
            new_vigenere_cipher.build_lookup_table_from_key();
            return new_vigenere_cipher;
        }

        fn build_lookup_table_from_key(&mut self) {
            let key_chars       = self.key.chars();
            let mut char_index  = 0;
            for character in key_chars {
                let mut index_hashmap: HashMap<char,char> = HashMap::new();
                let a_index = (character as u8) - ('a' as u8);
                for j in 0..26 {
                    println!("Doing {}",(j + ('a' as u8)) as char);
                    index_hashmap.insert((j + ('a' as u8)) as char, 
                                         (((a_index + j) %26) + ('a' as u8)) as char);
                }
                self.lookup_table.insert(char_index, index_hashmap);
                char_index += 1;
            }
        }

        fn extract_encrypted_char(&self, index: usize, character: char) -> Option<char> {
            let index = index % self.key.chars().count();
            match self.lookup_table.get(&index) {
                Some(table) => match table.get(&character) {
                    Some(encrypted_char) => Some(*encrypted_char),
                    None                 => None,
                },
                None        => None
            }
        }

    }
    impl Cipher for VigenereCipher {
        fn encrypt(&self, plaintext: String) -> Result<String, String> {
            let mut char_index = 0;
            let mut encrypted_str = String::new();
            for character in plaintext.chars() {
                match self.extract_encrypted_char(char_index, character) {
                    Some(character) => { encrypted_str.push(character); },
                    None            => { return Err(format!("Invalid character: {}", character)) },
                }
                char_index += 1;
            }
            return Ok(encrypted_str);
        }

        fn decrypt(&self, encrypted_text: String) -> Result<String, String> {
            unimplemented!();
        }

        fn bash(&self, encrypted_text: String) -> Result<String, String> {
            unimplemented!();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::vigenere;

    #[test]
    fn it_works() {
        let v = vigenere::VigenereCipher::new("golf");
        assert_eq!(v.lookup_table.get(&(0 as usize)).unwrap().get(&'a'), Some(&'g'));
        assert_eq!(v.lookup_table.get(&(0 as usize)).unwrap().get(&'c'), Some(&'i'));
        assert_eq!(v.lookup_table.get(&(1 as usize)).unwrap().get(&'c'), Some(&'q'));
    }

    #[test]
    fn encrypt_test() {
        let v = vigenere::VigenereCipher::new("ok");
        assert_eq!(v.encrypt("everyexercise".to_string()),Ok("sfsbmolofmwcs".to_string()));
    }
}
