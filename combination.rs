
const MOD: u64 = 998_244_353;


struct Combination::<const MOD: u64> {
    fp: Vec<u64>,
    rfp: Vec<u64>,
}
impl Combination {
    fn new(n: usize) -> Self {
        let mut fp = vec![1u64; n+1];
        for i in 1..n+1 {
            fp[i] = fp[i-1] * i as u64 % MOD;
        }
        let mut rfp = vec![1u64; n+1];
        rfp[n] = Self::mod_pow(fp[n], MOD - 2);
        for i in (1..n+1).rev() {
            rfp[i - 1] = rfp[i] * i as u64 % MOD;
        }
        return Combination {
            fp,
            rfp,
        }
    }

    fn calc(&self, n: usize, k: usize) -> u64 {
        if n < k {
            return 0;
        }
        self.fp[n] * self.rfp[k] % MOD * self.rfp[n-k] % MOD
    }

    fn mod_pow(mut a: u64, mut b: u64) -> u64 {
        let mut x = 1u64;
        while b > 0 {
            if b & 1 == 1 {
                x = x * a % MOD;
            }
            a = a * a % MOD;
            b >>= 1;
        }
        x
    }
}
