use crate::errors::ValueError;
use ibig::{ubig, UBig};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct FieldElement {
    pub num: UBig,
    pub prime: UBig,
}

pub trait FieldElementOps {
    fn new(num: UBig, prime: UBig) -> Result<FieldElement, ValueError>;
    fn pow(self, power: i128) -> Self;
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num + rhs.num) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: Self) -> Self {
        let new_cloned = self.prime.clone();
        let ret_cloned = self.prime.clone();
        assert_eq!(self.prime.clone(), rhs.prime);
        let new_num = if self.num < rhs.num {
            self.prime - ((rhs.num - self.num) % new_cloned)
        } else {
            self.num - rhs.num
        };
        FieldElement {
            num: new_num,
            prime: ret_cloned,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> Self {
        let new_cloned = self.prime.clone();
        let ret_cloned = self.prime.clone();
        FieldElement {
            num: self.prime - (self.num % new_cloned),
            prime: ret_cloned,
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: Self) -> Self {
        let ret_cloned = self.prime.clone();
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num * rhs.num) % self.prime,
            prime: ret_cloned,
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;
    fn div(self, rhs: Self) -> Self {
        let ret_cloned = self.prime.clone();
        assert_eq!(self.prime, rhs.prime);
        FieldElement {
            num: (self.num / rhs.num) % self.prime,
            prime: ret_cloned,
        }
    }
}

impl FieldElementOps for FieldElement {
    fn new(num: UBig, prime: UBig) -> Result<FieldElement, ValueError> {
        match num >= prime {
            true => Err(ValueError {
                message: format!("num {} not in field range 0 to {}", num, prime - ubig!(1)),
            }),
            false => Ok(FieldElement { num, prime }),
        }
    }
    fn pow(self, power: i128) -> Self {
        let exp = if power < 0 {
            1_usize + (-1 * power) as usize
        } else {
            power as usize
        };
        FieldElement {
            // num: self.num.pow(power) % self.prime,
            num: self.num.pow(exp) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let a = FieldElement::new(ubig!(3), ubig!(13));
        let b = FieldElement::new(ubig!(13), ubig!(13));
        let c = FieldElement::new(ubig!(14), ubig!(13));
        let d = FieldElement::new(ubig!(0), ubig!(13));
        assert!(a.is_ok());
        assert!(b.is_err());
        assert!(c.is_err());
        assert!(d.is_ok());
    }

    #[test]
    fn test_ne() {
        let a = FieldElement::new(ubig!(2), ubig!(13)).unwrap();
        let b = FieldElement::new(ubig!(10), ubig!(13)).unwrap();
        let c = FieldElement::new(ubig!(2), ubig!(13)).unwrap();
        assert_eq!(a, c);
        assert!(a != b);
        assert!(b != c);
    }

    #[test]
    fn test_add() {
        let mut a = FieldElement::new(ubig!(2), ubig!(13)).unwrap();
        let mut b = FieldElement::new(ubig!(10), ubig!(13)).unwrap();
        assert_eq!(a + b, FieldElement::new(ubig!(12), ubig!(13)).unwrap());
        a = FieldElement::new(ubig!(5), ubig!(13)).unwrap();
        b = FieldElement::new(ubig!(12), ubig!(13)).unwrap();
        assert_eq!(a + b, FieldElement::new(ubig!(4), ubig!(13)).unwrap());
    }

    #[test]
    fn test_sub() {
        let mut a = FieldElement::new(ubig!(10), ubig!(13)).unwrap();
        let mut b = FieldElement::new(ubig!(2), ubig!(13)).unwrap();
        assert_eq!(a - b, FieldElement::new(ubig!(8), ubig!(13)).unwrap());
        a = FieldElement::new(ubig!(5), ubig!(13)).unwrap();
        b = FieldElement::new(ubig!(12), ubig!(13)).unwrap();
        assert_eq!(a - b, FieldElement::new(ubig!(6), ubig!(13)).unwrap());
    }

    #[test]
    fn test_mul() {
        let mut a = FieldElement::new(ubig!(11), ubig!(13)).unwrap();
        let mut b = FieldElement::new(ubig!(2), ubig!(13)).unwrap();
        assert_eq!(a * b, FieldElement::new(ubig!(9), ubig!(13)).unwrap());
        a = FieldElement::new(ubig!(5), ubig!(13)).unwrap();
        b = FieldElement::new(ubig!(12), ubig!(13)).unwrap();
        assert_eq!(a * b, FieldElement::new(ubig!(8), ubig!(13)).unwrap());
    }

    #[test]
    fn test_pow() {
        let mut a = FieldElement::new(ubig!(7), ubig!(13)).unwrap();
        assert_eq!(a.pow(4), FieldElement::new(ubig!(9), ubig!(13)).unwrap());

        a = FieldElement::new(ubig!(7), ubig!(13)).unwrap();
        let b = FieldElement::new(ubig!(3), ubig!(13)).unwrap();
        assert_eq!(
            a.pow(3) * b,
            FieldElement::new(ubig!(2), ubig!(13)).unwrap()
        );
    }

    #[test]
    fn test_div() {
        let mut a = FieldElement::new(ubig!(3), ubig!(31)).unwrap();
        let mut b = FieldElement::new(ubig!(24), ubig!(31)).unwrap();
        assert_eq!(a / b, FieldElement::new(ubig!(4), ubig!(31)));
        a = FieldElement::new(ubig!(17), ubig!(31)).unwrap();
        assert_eq!(a.pow(-3), FieldElement::new(ubig!(29), ubig!(31)));
        a = FieldElement::new(ubig!(4), ubig!(31)).unwrap();
        b = FieldElement::new(ubig!(11), ubig!(31)).unwrap();
        assert_eq!(a.pow(-3) * b, FieldElement::new(ubig!(13), ubig!(31)));
        // println!("{:?}", a.pow(-3) * b);
        // println!("{:?}", FieldElement::new(ubig!(12), ubig!(31)));
    }
}
