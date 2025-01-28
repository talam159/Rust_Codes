use num_bigint::{BigInt, RandBigInt};
use num_integer::{gcd, lcm};
use num_traits::{One, Zero};
use rand::thread_rng;

fn main() {
    // Generate two 1024-bit primes p and q
    let p = generate_prime(1024);
    let q = generate_prime(1024);

    println!("p = {}", p);
    println!("q = {}", q);

    // Compute n, phi(n), and psi(n)
    let n = &p * &q;
    let phi_n = (&p - BigInt::one()) * (&q - BigInt::one());
    let psi_n = lcm(&p - BigInt::one(), &q - BigInt::one());

    println!("n = {}", n);
    println!("phi(n) = {}", phi_n);
    println!("psi(n) = {}", psi_n);

    // Verify gcd(e, psi(n)) = 1
    let e = BigInt::from(3);
    assert_eq!(gcd(e.clone(), psi_n.clone()), BigInt::one(), "e is not coprime to psi(n)");

    // Compute d1 and d2
    let d1 = mod_inverse(&e, &phi_n).expect("d1 does not exist");
    let d2 = mod_inverse(&e, &psi_n).expect("d2 does not exist");

    println!("d1 = {}", d1);
    println!("d2 = {}", d2);

    // Verify d1 and d2
    assert_eq!((&e * &d1) % &phi_n, BigInt::one(), "d1 is incorrect");
    assert_eq!((&e * &d2) % &psi_n, BigInt::one(), "d2 is incorrect");
}

/// Generate a prime number of the specified bit length
fn generate_prime(bits: usize) -> BigInt {
    let mut rng = thread_rng();
    loop {
        let candidate = rng.gen_bigint(bits as u64);
        if is_prime(&candidate) {
            return candidate;
        }
    }
}

/// Check if a number is prime (probabilistic test)
fn is_prime(n: &BigInt) -> bool {
    // Use a simple probabilistic test (e.g., Miller-Rabin)
    // For simplicity, we use a small number of iterations
    n.is_probable_prime(10)
}

/// Compute the modular inverse of `a` modulo `m`
fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g != BigInt::one() {
        None // Inverse does not exist if gcd(a, m) != 1
    } else {
        Some((x % m + m) % m) // Ensure the result is positive
    }
}

/// Extended Euclidean Algorithm to find gcd(a, b) and coefficients x, y such that ax + by = gcd(a, b)
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = extended_gcd(&(b % a), a);
        (g, y - (b / a) * x.clone(), x)
    }
}