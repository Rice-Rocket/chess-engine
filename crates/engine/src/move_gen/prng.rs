pub struct PRNG {
    seed: u64,
}

impl PRNG {
    pub fn new(seed: u64) -> PRNG {
        PRNG { seed }
    }

    pub fn gen(&mut self) -> u64 {
        self.rand_change()
    }

    pub fn sparse_rand(&mut self) -> u64 {
        let mut s = self.rand_change();
        s &= self.rand_change();
        s &= self.rand_change();
        s
    }

    pub fn singular_bit(&mut self) -> u64 {
        #[allow(clippy::transmute_num_to_bytes)]
        let bits: [u8; 8] = unsafe { std::mem::transmute(self.gen() ^ self.gen()) };
        let byte: u8 = bits.iter().fold(0, |acc, &x| acc ^ x);
        1u64.wrapping_shl((byte >> 2) as u32)
    }

    pub fn rand_change(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        self.seed.wrapping_mul(2_685_821_657_736_338_717)
    }
}

impl Iterator for PRNG {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gen())
    }
}
