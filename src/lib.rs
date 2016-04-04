pub mod vigenere {
    use std::collections::HashMap;

    struct VigenereCipher {
        pub key: String,
        lookup_table: HashMap<usize, HashMap<char, char>>,
    }

    impl VigenereCipher {

        /// Return a new VigenereCipher with a blank key by default
        pub fn new() -> VigenereCipher {
            let new_vigenere_cipher = VigenereCipher {
                key: String::from(""),
                lookup_table: HashMap::new(),
            };
            new_vigenere_cipher.build_lookup_table_from_key();
            return new_vigenere_cipher;
        }

        fn build_lookup_table_from_key(&self) {
            let character_count = self.key.chars().count();
            for i in 0..character_count {
                let index_hashmap: HashMap<char,char> = HashMap::new();
            }
        }

        pub fn encrypt(plaintext: String) -> String {

        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
