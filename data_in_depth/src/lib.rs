mod cpu;
mod memory;

#[test]
fn transmute_test() {
    let a: f32 = 42.42;
    println!("{}", a);

    let b: i32 = unsafe { std::mem::transmute(a) };
    println!("{:032b}", b);

    let a: f32 = unsafe { std::mem::transmute(b) };
    println!("{}", a);
}

#[allow(arithmetic_overflow)]
#[test]
fn integer_overflow() {
    let a: u8 = 200;
    let b: u8 = 200;
    let c: u8 = a + b;

    println!("{}", c);
}

#[test]
fn flow_test() {
    let mut x;

    x = 42;

    let y = &x;

    // invalid assignment since x is borrowed by y
    // x = 43;

    assert_eq!(*y, 42);
}

#[test]
fn endianness() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let lit_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(lit_endian) };

    println!("{} vs. {}", a, b);
}

#[test]
fn float_fun() {
    let n = 42.42;
    let (sign_bits, exponent_bits, mantissa_bits) = to_parts(n);
    let (sign, exponent, mantissa) = decode((sign_bits, exponent_bits, mantissa_bits));

    let n_ = from_parts((sign, exponent, mantissa));

    println!("{} -> {}", n, n_);
    println!("field    | as_bits | as real number");
    println!("sign     | {:01b}  | {}", sign_bits, sign);
    println!("exponent | {:08b}  | {}", exponent_bits, exponent);
    println!("mantissa | {:23b}  | {}", mantissa_bits, mantissa);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
    let n_bits = n.to_bits();
    let sign = n_bits >> 31;

    let exponent_bits = n_bits >> 23;
    let exponent_bits = exponent_bits & 0xFF;

    let mantissa_bits = n_bits & 0x7FFFFFF;

    (sign, exponent_bits, mantissa_bits)
}

const EXP_BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn decode((sign, exponent, fraction): (u32, u32, u32)) -> (f32, f32, f32) {
    let sign = (-1.0_f32).powf(sign as f32);

    let exponent = exponent as i32 - EXP_BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let bit_at_i = fraction & mask;

        if bit_at_i != 0 {
            let i_ = i as f32;
            let weight = RADIX.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (sign, exponent, mantissa)
}

fn from_parts((sign, exponent, mantissa): (f32, f32, f32)) -> f32 {
    sign * exponent * mantissa
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n >= 1.0 {
            Q7(i8::MAX)
        } else if n <= -1.0 {
            Q7(i8::MIN)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod q7_tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7::from(1.0));
        assert_eq!(Q7::from(-10.0), Q7::from(-1.0));
    }
}

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0x3F000000;
    let large_n = (n as u32) << 15; // 23 - 8
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);

    println!("{}", m);

    2.0 * (m - 0.5)
}

#[test]
fn mock_rand_test() {
    let x = mock_rand(42);

    assert_eq!(x, 0.1640625);
}
