use super::encoding::{decode, encode};
use rand::RngExt;
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ShortUID(NonZeroU64);

impl ShortUID {
    const NUMBER_MAX: u64 = 99_999_999_999;

    const LETTERS_COUNT: usize = 5;
    const LETTERS_BASE: u64 = 26;
    // const LETTERS_SPACE: u64 = 11_881_376; // 26^5

    pub fn new() -> Self {
        let mut rng = rand::rng();

        let number = rng.random_range(1..=Self::NUMBER_MAX);

        let letters = (0..Self::LETTERS_COUNT).fold(0_u64, |n, _| {
            n * Self::LETTERS_BASE + rng.random_range(0..Self::LETTERS_BASE)
        });

        Self(NonZeroU64::new(encode(number, letters)).unwrap())
    }

    pub fn validate(value: &str) -> Option<Self> {
        let b = value.as_bytes();

        // Format: NNlll-NNNNll-NNNNN
        if b.len() != 18
            || b[5] != b'-'
            || b[12] != b'-'
            || !b[0..2].iter().all(u8::is_ascii_digit)
            || !b[2..5].iter().all(u8::is_ascii_lowercase)
            || !b[6..10].iter().all(u8::is_ascii_digit)
            || !b[10..12].iter().all(u8::is_ascii_lowercase)
            || !b[13..18].iter().all(u8::is_ascii_digit)
        {
            return None;
        }

        let number = [b[0], b[1], b[6], b[7], b[8], b[9], b[13], b[14], b[15]]
            .iter()
            .fold(0_u64, |n, &d| n * 10 + (d - b'0') as u64);

        let letters = [b[2], b[3], b[4], b[10], b[11]]
            .iter()
            .fold(0_u64, |n, &c| n * 26 + (c - b'a') as u64);

        if number == 0 || suffix == 0 {
            return None;
        }

        Some(Self(
            NonZeroU64::new(encode(number, letters)).unwrap(),
        ))
    }
    pub fn as_u64(&self) -> u64 {
        self.0.get()
    }

    pub fn as_bytes(&self) -> [u8; 8] {
        self.0.get().to_be_bytes()
    }

    pub fn number(&self) -> i32 {
        -(decode(self.0.get()).0 as i32)
    }
}
