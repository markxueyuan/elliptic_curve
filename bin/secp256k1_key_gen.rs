use elliptic_curve::secp256k1::SECP256K1;

fn main() {
    let secp256k1 = SECP256K1::new();
    let secret = secp256k1.get_secret_key();
    let public = secp256k1.get_pubkey_str(secret);
    let secret = secret.to_string();
    println!("secret key: {}", secret);
    println!("public key: {}", public);
}