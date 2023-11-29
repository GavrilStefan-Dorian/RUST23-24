fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}

use std::{
    fmt,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T, U>(x: T, y: U) -> Self
    where
        f64: From<T>,
        f64: From<U>,
        T: Copy,
        U: Copy,
    {
        Complex {
            real: f64::from(x),
            imag: f64::from(y),
        }
    }

    fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl From<i32> for Complex {
    fn from(x: i32) -> Complex {
        Complex::new(x, 0)
    }
}

impl From<f64> for Complex {
    fn from(x: f64) -> Complex {
        Complex::new(x, 0)
    }
}

impl<X> Add<X> for Complex
where
    Complex: From<X>,
    X: Copy,
{
    type Output = Complex;

    fn add(self, rhs: X) -> Self::Output {
        Complex::new(
            self.real + Complex::from(rhs).real,
            self.imag + Complex::from(rhs).imag,
        )
    }
}

impl<X> Sub<X> for Complex
where
    Complex: From<X>,
    X: Copy,
{
    type Output = Complex;

    fn sub(self, rhs: X) -> Self::Output {
        Complex::new(
            self.real - Complex::from(rhs).real,
            self.imag - Complex::from(rhs).imag,
        )
    }
}

impl<X> Mul<X> for Complex
where
    Complex: From<X>,
    X: Copy,
{
    type Output = Complex;

    fn mul(self, rhs: X) -> Self::Output {
        Complex::new(
            self.real * Complex::from(rhs).real - self.imag * Complex::from(rhs).imag,
            self.real * Complex::from(rhs).imag + self.imag * Complex::from(rhs).real,
        )
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex::new(-1f64 * self.real, -1f64 * self.imag)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.real as i32;
        let y = self.imag as i32;

        if x == 0 {
            if y == 0 {
                write!(f, "0")
            } else {
                if y > 0 {
                    write!(f, "{}i", y)
                } else {
                    write!(f, "-{}i", -1 * y)
                }
            }
        } else {
            if y == 0 {
                write!(f, "{}", x)
            } else {
                if y > 0 {
                    write!(f, "{}+{}i", x, y)
                } else {
                    write!(f, "{}-{}i", x, -1 * y)
                }
            }
        }
    }
}
