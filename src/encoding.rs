const LETTERS_SPACE: u64 = 11_881_376;

const NUMBER_BITS: u32 = 37;
const LETTERS_BITS: u32 = 24;

pub(super) fn encode(number: u64, letters: u64) -> u64 {
    number << u64::BITS - NUMBER_BITS | (letters << u64::BITS - NUMBER_BITS - LETTERS_BITS)
}

pub(super) fn decode(value: u64) -> (u64, u64) {
    let letters = value & ((1 << (u64::BITS - NUMBER_BITS)) - 1);

    let number = value >> (u64::BITS - NUMBER_BITS);

    (number, letters)
}
