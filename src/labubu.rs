//! Labubu Assets
//!
//! Labubu module
//!
//! This module builds a `Labubu Asset` from a funded address generated in the user's browser.

use secp256k1::{Keypair, SecretKey, SECP256K1};

/// TODO(@luisschwab): remove later (sepc256k1 test)
#[cfg(test)]
mod tests {
    use secp256k1::{rand::thread_rng, Keypair, SecretKey, SECP256K1};

    #[test]
    fn secp256k1() {
        let mut rng = thread_rng();

        let sk = SecretKey::new(&mut rng);
        let keypair = Keypair::from_secret_key(SECP256K1, &sk);
        let pk = keypair.public_key();
        println!("{:?}", sk);
        println!("{:?}", pk);
    }
}
