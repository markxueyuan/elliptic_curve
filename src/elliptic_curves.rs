use finite_field::{FieldElement};
use std::ops::{Add, Div, Mul, Rem, Sub, Shr, BitAnd};
use num::{One, Zero, Num, Bounded};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Coords<T> {
    Some(FieldElement<T>, FieldElement<T>),
    Identity,
}

use Coords::{Some, Identity};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EllipticCurve<T> {
    pub a: FieldElement<T>,
    pub b: FieldElement<T>,
}

impl<T> EllipticCurve<T> {
    pub fn new(a: FieldElement<T>, b: FieldElement<T>) -> EllipticCurve<T> {
        EllipticCurve {
            a,
            b    
        }    
    }     
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    coords: Coords<T>,
    curve : EllipticCurve<T>    
}

impl<T> Point<T> 
    where T: Rem<Output = T> + Mul<Output = T> + Copy + Sub<Output = T> + Add<Output = T> + Shr<Output = T>,
          T: One + Num + PartialOrd + Bounded + Debug,
{
    pub fn new(coords: Coords<T>, curve: EllipticCurve<T>) -> Point<T> {
        let a = curve.a;
        let b = curve.b;    
        let one: T = One::one();
        let two = one + one;
        let three = two + one;

        if let Some(x, y) = coords {
            assert_eq!(y.pow(two), x.pow(three) + a * x + b);    
        }

        Point {
            coords,
            curve    
        }          
    }    
}

impl<T> Add for Point<T> 
    where T: PartialEq,
          T: PartialOrd + Debug + Sub<Output = T> + Rem<Output = T> + Bounded,
          T: Zero + Copy + Div<Output = T> + Num + Shr<T, Output = T> + One,
{
    type Output = Self;    
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.curve, rhs.curve);  
        let a = self.curve.a;
        let one: T = One::one();
        let two = one + one;
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

pub struct Scalar<T>(T);

impl<T> Mul<Point<T>> for Scalar<T> 
    where T: Shr + Zero,
          T: Rem<Output = T> + Mul<Output = T> + Copy + Sub<Output = T> + Add<Output = T> + Shr<Output = T>,
          T: One + Num + PartialOrd + Bounded + Debug + BitAnd<Output = T>,
{
    type Output = Point<T>;
    
    fn mul(self, rhs: Point<T>) -> Self::Output {   
        let mut coef = self.0;
        let zero: T = Zero::zero();
        let one: T = One::one();
        assert!(coef >= zero);

        let mut current = rhs;
        let mut result = Point::new(Identity, rhs.curve);
        while coef > zero {
            if coef & one > zero {                  
                result = result + current;
            }
            current = current + current;
            coef = coef >> one;
        }

        result        
    }    
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_works() {
        //////// Two points
        // point 1
        let x: FieldElement<u16> = FieldElement::new(192, 223);
        let y: FieldElement<u16> = FieldElement::new(105, 223);
        let a: FieldElement<u16> = FieldElement::new(0, 223);
        let b: FieldElement<u16> = FieldElement::new(7, 223);
        let coords = Some(x, y);
        let curve = EllipticCurve {a, b};
        let point = Point::new(coords, curve);

        // point 2        
        let x: FieldElement<u16> = FieldElement::new(170, 223);
        let y: FieldElement<u16> = FieldElement::new(142, 223);
        let a: FieldElement<u16> = FieldElement::new(0, 223);
        let b: FieldElement<u16> = FieldElement::new(7, 223);
        let coords = Some(x, y);
        let curve = EllipticCurve {a, b};
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
        ///////// Two inverse points
        // point 1
        let x: FieldElement<u16> = FieldElement::new(69, 223);
        let y: FieldElement<u16> = FieldElement::new(86, 223);
        let a: FieldElement<u16> = FieldElement::new(0, 223);
        let b: FieldElement<u16> = FieldElement::new(7, 223);
        let coords = Some(x, y);
        let curve = EllipticCurve {a, b};
        let point = Point::new(coords, curve);

        // point 2        
        let x: FieldElement<u16> = FieldElement::new(69, 223);
        let y: FieldElement<u16> = FieldElement::new(137, 223);
        let a: FieldElement<u16> = FieldElement::new(0, 223);
        let b: FieldElement<u16> = FieldElement::new(7, 223);
        let coords = Some(x, y);
        let curve = EllipticCurve {a, b};
        let point2 = Point::new(coords, curve); 

        // Zero point
        let zero = Point::new(Identity, curve);
        
        // Adding two inverse points leads to zero.
        assert_eq!(point + point2, zero);

        // Adding zero to a point leads to the same point,
        assert_eq!(zero + point, point);
        assert_eq!(zero + point2, point2);     
    }

    #[test]
    fn scalar_mul_works() {
        // define a point on an elliptic curve
        let x: FieldElement<u16> = FieldElement::new(47, 223);
        let y: FieldElement<u16> = FieldElement::new(71, 223);
        let a: FieldElement<u16> = FieldElement::new(0, 223);
        let b: FieldElement<u16> = FieldElement::new(7, 223);
        let coords = Some(x, y);
        let curve = EllipticCurve {a, b};
        let point = Point::new(coords, curve);

        // scalar multiplication of the point on the elliptic curve
        let four = Scalar(4 as u16);
        let point2 = four * point;         
        println!("{:?}", point2);               

        let twenty_one = Scalar(21 as u16);
        let point3 = twenty_one * point;

        // Zero point
        let zero = Point::new(Identity, curve);
        assert_eq!(point3, zero);        
    }    
}
