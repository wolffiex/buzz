mod buzz;

use std::iter::successors;

extern crate ring;

use ring::{digest, pbkdf2};
use std::{collections::HashMap, num::NonZeroU32};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

enum Error {
    WrongUsernameOrPassword,
}

struct PasswordDatabase {
    pbkdf2_iterations: NonZeroU32,
    db_salt_component: [u8; 16],

    // Normally this would be a persistent database.
    storage: HashMap<String, Credential>,
}

impl PasswordDatabase {
    pub fn store_password(&mut self, username: &str, password: &str) {
        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            self.pbkdf2_iterations,
            &salt,
            password.as_bytes(),
            &mut to_store,
        );
        self.storage.insert(String::from(username), to_store);
    }

    pub fn verify_password(&self, username: &str, attempted_password: &str) -> Result<(), Error> {
        match self.storage.get(username) {
            Some(actual_password) => {
                let salt = self.salt(username);
                pbkdf2::verify(
                    PBKDF2_ALG,
                    self.pbkdf2_iterations,
                    &salt,
                    attempted_password.as_bytes(),
                    actual_password,
                )
                .map_err(|_| Error::WrongUsernameOrPassword)
            }

            None => Err(Error::WrongUsernameOrPassword),
        }
    }

    // The salt should have a user-specific component so that an attacker
    // cannot crack one password for multiple users in the database. It
    // should have a database-unique component so that an attacker cannot
    // crack the same user's password across databases in the unfortunate
    // but common case that the user has used the same password for
    // multiple systems.
    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() + username.as_bytes().len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }
}

fn main() {
    // Normally these parameters would be loaded from a configuration file.
    let mut db = PasswordDatabase {
        pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
        db_salt_component: [
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39,
            0x01, 0x8a,
        ],
        storage: HashMap::new(),
    };

    db.store_password("alice", "@74d7]404j|W}6u");

    // An attempt to log in with the wrong password fails.
    assert!(db.verify_password("alice", "wrong password").is_err());

    // Normally there should be an expoentially-increasing delay between
    // attempts to further protect against online attacks.

    // An attempt to log in with the right password succeeds.
    assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
}

// fn not_main() {
//     println!("Hello, world!");
//     // The password will be used to generate a key
//     let password = b"nice password";
//
//     // Usually the salt has some random data and something that relates to the user
//     // like an username
//     let salt = [0, 1, 2, 3, 4, 5, 6, 7];
//
//     // Keys are sent as &[T] and must have 32 bytes
//     let mut key = [0; 32];
//     derive(PBKDF2_HMAC_SHA256, NonZeroU32::new(10).unwrap(),
//            &salt, &password[..], &mut key);
//
//     // Your private data
//     let content = b"content to encrypt".to_vec();
//     println!("Content to encrypt's size {}", content.len());
//
//     // Additional data that you would like to send and it would not be encrypted but it would
//     // be signed
//     let additional_data: [u8; 0] = [];
//
//     // Ring uses the same input variable as output
//     let mut in_out = content.clone();
//
//     // The input/output variable need some space for a suffix
//     println!("Tag len {}", CHACHA20_POLY1305.tag_len());
//     for _ in 0..CHACHA20_POLY1305.tag_len() {
//         in_out.push(0);
//     }
//
//     // Opening key used to decrypt data
//     let opening_key = OpeningKey::new(CHACHA20_POLY1305, &key).unwrap();
//
//     // Sealing key used to encrypt data
//     let sealing_key = SealingKey::new(&CHACHA20_POLY1305, &key).unwrap();
//
//     // Random data must be used only once per encryption
//     let mut nonce = vec![0; 12];
//
//     // Fill nonce with random data
//     let rand = SystemRandom::new();
//     rand.fill(&mut nonce).unwrap();
//
//     // Encrypt data into in_out variable
//     let output_size = seal_in_place(&sealing_key, &nonce, &additional_data, &mut in_out,
//                                     CHACHA20_POLY1305.tag_len()).unwrap();
//
//     println!("Encrypted data's size {}", output_size);
//
//     let decrypted_data = open_in_place(&opening_key, &nonce, &additional_data,
//                                        0, &mut in_out).unwrap();
//
//     println!("{:?}", String::from_utf8(decrypted_data.to_vec()).unwrap());
//     assert_eq!(content, decrypted_data);
//     //let private_key = ??
// }

#[cfg(test)]
#[test]
fn test_loop() {
    let result: usize = successors(Some(0), |i| if *i < 10 { Some(*i + 1) } else { None })
        .last()
        .unwrap();
    assert_eq!(10, result);
}
