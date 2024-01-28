use finite_field::FieldElementBig;
use std::ops::{Add, Mul};
use crypto_bigint::Uint;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Coords<const LIMBS: usize> {
    Some(FieldElementBig<LIMBS>, FieldElementBig<LIMBS>),
    Identity,
}

use Coords::{Some, Identity};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EllipticCurve<const LIMBS: usize>{
    pub a: FieldElementBig<LIMBS>,
    pub b: FieldElementBig<LIMBS>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<const LIMBS: usize> {
    pub coords: Coords<LIMBS>,
    pub curve : EllipticCurve<LIMBS>    
}

impl<const LIMBS: usize> EllipticCurve<LIMBS> {
    pub fn new(a: FieldElementBig<LIMBS>, b: FieldElementBig<LIMBS>) -> EllipticCurve<LIMBS> {
        EllipticCurve {
            a,
            b
        }
    }  
}

impl<const LIMBS: usize> Point<LIMBS> {
    pub fn new(coords: Coords<LIMBS>, curve: EllipticCurve<LIMBS>) -> Point<LIMBS> {
        let two = Uint::from(2u8);
        let three = Uint::from(3u8);
        let a = curve.a;
        let b = curve.b;
        if let Some(x, y) = coords {
            assert_eq!(y.pow(two), x.pow(three) + a * x + b);     
        }    

        Point {
            coords,
            curve    
        }        
    }    
}

impl<const LIMBS: usize> Add for Point<LIMBS> {
    type Output = Point<LIMBS>;
    fn add(self, rhs: Self) -> Point<LIMBS> {
        assert_eq!(self.curve, rhs.curve);
        let a = self.curve.a;
        let two: Uint<LIMBS> = Uint::from(2u8);
        if let Some(x1, y1) = self.coords {
            if let Some(x2, y2) = rhs.coords {
                if x1 != x2 {
                    let s = (y2 - y1) / (x2 - x1); 
                    let x3 = s.pow(two) - x1 - x2;
                    let y3 = s * (x1 -x3) - y1;
                    return Point::new(Some(x3, y3), self.curve);
                } else if x1 == x2 && y1 == y2 {
                    let s = (x1.pow(two) + x1.pow(two) + x1.pow(two) + a) / (y1 +  y1);
                    let x3 = s.pow(two) - x1 - x1;
                    let y3 = s * (x1 - x3) - y1;
                    return Point::new(Some(x3, y3), self.curve);
                }
            } else {
                return self;    
            } 
        } else {
            if let Some(_x2, _y2) = rhs.coords {
                return rhs;    
            }    
        }
        Point::new(Identity, self.curve)        
    }    
}

impl<const LIMBS: usize> Mul<Point<LIMBS>> for Uint<LIMBS> {
    type Output = Point<LIMBS>;
    fn mul(self, rhs: Point<LIMBS>) -> Self::Output {
        let mut coef = self;
        let zero = Uint::ZERO;
        let one = Uint::ONE;
        assert!(coef >= zero);

        let mut current = rhs;
        let mut result = Point::new(Identity, rhs.curve);

        while coef > zero {
            if coef & one > zero {
                result = result + current;    
            }    
            current = current + current;
            coef = coef >> (1_usize);
        }                

        result        

         

    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::U256;

    #[test]
    fn curve_works() {

        ////////////////// Curve
        let a = FieldElementBig::new(U256::from(0u8), U256::from(223u8));
        let b = FieldElementBig::new(U256::from(7u8), U256::from(223u8));
        let curve = EllipticCurve {a, b};
        

        ////////////////// Two points
        // point 1 
        let x = FieldElementBig::new(U256::from(192u8), U256::from(223u8));
        let y = FieldElementBig::new(U256::from(105u8), U256::from(223u8));
        let coords = Some(x, y);

        let point = Point::new(coords, curve);

        // point 2 
        let x = FieldElementBig::new(U256::from(170u8), U256::from(223u8));
        let y = FieldElementBig::new(U256::from(142u8), U256::from(223u8));
        let coords = Some(x, y);

        let point2 = Point::new(coords, curve);

        // Adding two different points
        let point3 = point + point2;
        println!("{:?}", point3);

        // Adding two points of the same value
        let point4 = point + point;
        println!("{:?}", point4);

        // associativity holds
        assert_eq!(point3 + point, point4 + point2);                      
    }


    #[test]
    fn identity_works() {
        ////////////////// Curve
        let a = FieldElementBig::new(U256::from(0u8), U256::from(223u8));
        let b = FieldElementBig::new(U256::from(7u8), U256::from(223u8));
        let curve = EllipticCurve {a, b};
        

        ////////////////// Two inverse points
        // point 1 
        let x = FieldElementBig::new(U256::from(69u8), U256::from(223u8));
        let y = FieldElementBig::new(U256::from(86u8), U256::from(223u8));
        let coords = Some(x, y);

        let point = Point::new(coords, curve);

        // point 2 
        let x = FieldElementBig::new(U256::from(69u8), U256::from(223u8));
        let y = FieldElementBig::new(U256::from(137u8), U256::from(223u8));
        let coords = Some(x, y);

        let point2 = Point::new(coords, curve);    

        // Zero point
        let zero = Point::new(Identity, curve);

        // Adding two inverse points lead to zero.
        assert_eq!(point + point2, zero);

        // Adding zero to a point leads to the same point
        assert_eq!(zero + point, point);
        assert_eq!(zero + point2, point2);                        
    }    

    #[test]
    fn scalar_mul_works() {
        ////////////////// Curve
        let a = FieldElementBig::new(U256::from(0u8), U256::from(223u8));
        let b = FieldElementBig::new(U256::from(7u8), U256::from(223u8));
        let curve = EllipticCurve {a, b};
        

        ////////////////// Two inverse points
        // point 1 
        let x = FieldElementBig::new(U256::from(47u8), U256::from(223u8));
        let y = FieldElementBig::new(U256::from(71u8), U256::from(223u8));
        let coords = Some(x, y);

        let point = Point::new(coords, curve);    

        // scalar multiplication of the point on the elliptic curve
        let four = U256::from(4u8);
        let point2 = four * point;
        println!("{:?}", point2);

        let twenty_one = U256::from(21u8);
        let point3 = twenty_one * point;

        // Zero point
        let zero = Point::new(Identity, curve);
        assert_eq!(point3, zero);                
    }    


}