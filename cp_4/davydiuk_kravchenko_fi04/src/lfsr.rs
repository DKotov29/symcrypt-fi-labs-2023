pub struct LFSR {
    filling: u32,
    mask_bit_to_pop: u32,
    mask_bit_to_set: u32,
    max_n_bit: u8,
    recurse: u32,
}

impl LFSR {
    pub fn new(r_coef: u32, poly_deg: u8) -> Self {
        let max_bit_n = poly_deg - 1;
        let mask_for_bit_to_set = 1 << max_bit_n;
        Self {
            filling: 0,
            mask_bit_to_pop: 1,
            mask_bit_to_set: mask_for_bit_to_set,
            max_n_bit: max_bit_n,
            recurse: r_coef,
        }
    }

    pub fn generate_from_fill(&mut self, initial_fill: u32, length: u64) -> Vec<u8> {
        self.filling = initial_fill;
        let mut feedback = Vec::with_capacity(length as usize);
        for _ in 0..length {
            feedback.push((self.filling & self.mask_bit_to_pop) as u8);
            self.filling = (self.filling >> 1)
                ^ (((self.filling & self.recurse).count_ones() & 1) << self.max_n_bit);
        }
        feedback
    }
}
