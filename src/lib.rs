use num_bigint::BigUint;

// output => n^exponet mod p
pub fn exponetiate(n: &BigUint, exponent: &BigUint, p: &BigUint) -> BigUint {
    n.modpow(exponent, p)
}

// output => s = k - c * x mod q
pub fn solve(k: &BigUint, c:&BigUint, x:&BigUint, q:&BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }

    return q - (c * x - k).modpow(&BigUint::from(1u32), q);
}

// cond1: r1 = alpha^s * y1^c mod p
// cond2: r2 = beta^s * y2^c mod p
pub fn verify(r1: &BigUint, r2:&BigUint, alpha: &BigUint, beta:&BigUint, y1:&BigUint, y2:&BigUint, s:&BigUint, c:&BigUint, p:&BigUint) -> bool {
    let cond1 = *r1 == alpha.modpow(s, p) * y1.modpow(c, p);
    let cond2 = *r2 == beta.modpow(s, p) * y2.modpow(c, p);

    cond1 && cond2
}