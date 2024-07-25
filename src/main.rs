use num_bigint::BigInt;
use std::str::FromStr;

// Function to perform modular multiplication
fn mod_mul(a: &BigInt, b: &BigInt, m: &BigInt) -> BigInt {
    (a * b) % m
}

fn main() {
    let a = BigInt::from_str("1234567890123456789012345678901234567890").unwrap();
    let b = BigInt::from_str("987654321098765432109876543210987654321").unwrap();
    let m = BigInt::from_str("999999999999999999999999999999999999999").unwrap();

    let result = mod_mul(&a, &b, &m);
    println!("The result of modular multiplication is: {}", result);
}

