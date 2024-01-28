use finite_field::FieldElementBig;
use crate::elliptic_curves_bigint::{Coords, EllipticCurve, Point};
use crypto_bigint::{U256, NonZero, RandomMod, rand_core::OsRng};
use num_bigint::BigUint;
use Coords::{Some};

pub struct SECP256K1 {
    pub p: String,
    pub gx: String,
    pub gy: String,
    pub n: String,
    pub a: u8,
    pub b: u8,    
}

impl SECP256K1 {
    pub fn new() -> SECP256K1 {
        SECP256K1 {
            p: Self::get_p(),
            gx: "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_owned(),
            gy: "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8".to_owned(),
            n: "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".to_owned(),
            a: 0u8,
            b: 7u8,
        }
    }    

    fn get_p() -> String {
        let n2 = BigUint::from(2u8);
        let n977 = BigUint::from(977u16);
        let p = n2.pow(256) - n2.pow(32) - n977;
        p.to_str_radix(16)
    }    

    pub fn get_order(&self) -> U256 {
        U256::from_be_hex(&self.p)   
    }
    pub fn get_curve(&self) -> EllipticCurve<4> {
        let a = U256::from(self.a);
        let b = U256::from(self.b);
        let p = self.get_order();
        let a = FieldElementBig::new(a, p);
        let b = FieldElementBig::new(b, p);
        EllipticCurve {a, b}
    }
    pub fn get_generator_coords(&self) -> Coords<4> {
        let gx = U256::from_be_hex(self.gx.as_str());
        let gy = U256::from_be_hex(self.gy.as_str());
        let p = self.get_order();
        let gx = FieldElementBig::new(gx, p);
        let gy = FieldElementBig::new(gy, p);
        Some(gx, gy)
    }   

    pub fn get_generator_point(&self) -> Point<4> {
        let curve = self.get_curve();
        let coords = self.get_generator_coords();
        Point::new(coords, curve)
    }

    pub fn get_group_order(&self) -> U256 {
        
        U256::from_be_hex(self.n.as_str()) 
    }    

    // generate a cryptographically secure random key less than n
    pub fn get_secret_key(&self) -> U256 {
        let n = self.get_group_order();
        let modulus = NonZero::new(n).unwrap();
        U256::random_mod(&mut OsRng, &modulus)    
    }    

    pub fn get_public_key(&self, secret_key: U256) -> Point<4> {
        let point = self.get_generator_point();
        secret_key * point    
    }    

    pub fn get_pubkey_str(&self, secret_key: U256) -> String {
        let public = self.get_public_key(secret_key);
        if let Some(x, y) =  public.coords {
            format!("{}, {}", x.get_num().to_string(), y.get_num().to_string())   
        } else {
            "ZERO".to_owned()    
        }
    }    
}




#[cfg(test)]
mod tests {
    use super::*;
    use Coords::Identity;

    #[test]
    fn secp256k1_works() {
        
        // get the generator poirnt G of secp256k1        
        let secp256k1 = SECP256K1::new();
        let point = secp256k1.get_generator_point();
        
        // get the group order n of secp256k1
        let group_order = secp256k1.get_group_order();


        // get an zero (identity / infinite) point of secp256k1 curve.
        let curve = secp256k1.get_curve();
        let zero = Point::new(Identity, curve);

        // It should be the case that n * G = 0
        assert_eq!(zero, group_order * point);
    }

    #[test]
    fn secret_key_works() {
        let secp256k1 = SECP256K1::new();    
        let secret = secp256k1.get_secret_key();
        println!("{:?}", secret);
    }    

    #[test]
    fn pub_key_works() {
        let secp256k1 = SECP256K1::new();    
        let secret = secp256k1.get_secret_key();
        let public = secp256k1.get_public_key(secret);
        println!("secret key: {:?}", secret);
        println!("public key: {:?}", public);
    }    
}


