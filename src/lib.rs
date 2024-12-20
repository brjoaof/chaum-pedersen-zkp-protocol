use num_bigint::{BigUint, RandBigInt};

pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
}

impl ZKP {
    // output => n^exponet mod p
    pub fn exponetiate(n: &BigUint, exponent: &BigUint, p: &BigUint) -> BigUint {
        n.modpow(exponent, p)
    }

    // output => s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }

        return &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q);
    }

    // cond1: r1 = alpha^s * y1^c mod p
    // cond2: r2 = beta^s * y2^c mod p
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        s: &BigUint,
        c: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        let cond2 = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);

        cond1 && cond2
    }

    pub fn generate_random_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_below(bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q,
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c: BigUint = BigUint::from(4u32);

        let y1 = ZKP::exponetiate(&alpha, &x, &p);
        let y2 = ZKP::exponetiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponetiate(&alpha, &k, &p);
        let r2 = ZKP::exponetiate(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &s, &c);
        assert!(result);

        // Fake Secret
        let x_fake = BigUint::from(7u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &s_fake, &c);
        assert!(!result);
    }

    #[test]
    fn test_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_below(&q);

        let c: BigUint = ZKP::generate_random_below(&q);

        let y1 = ZKP::exponetiate(&alpha, &x, &p);
        let y2 = ZKP::exponetiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponetiate(&alpha, &k, &p);
        let r2 = ZKP::exponetiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &s, &c);
        assert!(result);
    }

    #[test]
    fn test_1024_bits_constant() {
        //
        //    Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        //    The hexadecimal value of the prime is:
        //
        //    p = B10B8F96 A080E01D DE92DE5E AE5D54EC 52C99FBC FB06A3C6
        //        9A6A9DCA 52D23B61 6073E286 75A23D18 9838EF1E 2EE652C0
        //        13ECB4AE A9061123 24975C3C D49B83BF ACCBDD7D 90C4BD70
        //        98488E9C 219A7372 4EFFD6FA E5644738 FAA31A4F F55BCCC0
        //        A151AF5F 0DC8B4BD 45BF37DF 365C1A65 E68CFDA7 6D4DA708
        //        DF1FB2BC 2E4A4371
        //
        //    The hexadecimal value of the generator is:
        //
        //    g = A4D1CBD5 C3FD3412 6765A442 EFB99905 F8104DD2 58AC507F
        //        D6406CFF 14266D31 266FEA1E 5C41564B 777E690F 5504F213
        //        160217B4 B01B886A 5E91547F 9E2749F4 D7FBD7D3 B9A92EE1
        //        909D0D22 63F80A76 A6A24C08 7A091F53 1DBF0A01 69B6A28A
        //        D662A4D1 8E73AFA3 2D779D59 18D08BC8 858F4DCE F97C2A24
        //        855E6EEB 22B3B2E5
        //
        //    The generator generates a prime-order subgroup of size:
        //    q = F518AA87 81A8DF27 8ABA4E7D 64B7CB9D 49462353
        //
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(
            &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );

        // beta = alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_below(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = ZKP::generate_random_below(&q);
        let k = ZKP::generate_random_below(&q);

        let c: BigUint = ZKP::generate_random_below(&q);

        let y1 = ZKP::exponetiate(&alpha, &x, &p);
        let y2 = ZKP::exponetiate(&beta, &x, &p);

        let r1 = ZKP::exponetiate(&alpha, &k, &p);
        let r2 = ZKP::exponetiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &s, &c);
        assert!(result);
    }
}
