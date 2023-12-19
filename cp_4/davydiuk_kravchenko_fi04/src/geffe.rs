use crate::lfsr::LFSR;

pub struct Geffe {
    L1: LFSR,
    L2: LFSR,
    L3: LFSR,
}

impl Geffe {
    pub fn new(L1: LFSR, L2: LFSR, L3: LFSR) -> Self {
        Geffe { L1, L2, L3 }
    }

    pub fn generate(&mut self, init_l1: u32, init_l2: u32, init_l3: u32, length: usize) -> Vec<u8> {
        let mut res = vec![0u8; length];

        let X = self.L1.generate_from_fill(init_l1, length as u64);
        let Y = self.L2.generate_from_fill(init_l2, length as u64);
        let S = self.L3.generate_from_fill(init_l3, length as u64);
        for i in 0..length {
            if S[i] == 1 {
                res[i] = X[i];
            } else {
                res[i] = Y[i];
            }
        }
        res
    }
}
