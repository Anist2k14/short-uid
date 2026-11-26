const SUFFIX_MAX: u64 = 99;
const LETTERS_SPACE: u64 = 11_881_376;

const NUMBER_BITS: u32 = 30;
const SUFFIX_BITS: u32 = 7;
const LETTERS_BITS: u32 = 24;

pub(super) fn encode(number: u64, suffix: u64, letters: u64) -> u64 {
    (number << (SUFFIX_BITS + LETTERS_BITS)) | (suffix << LETTERS_BITS) | letters
}

pub(super) fn decode(value: u64) -> (u64, u64, u64) {
    let letters = value & ((1 << LETTERS_BITS) - 1);

    let suffix = (value >> LETTERS_BITS) & ((1 << SUFFIX_BITS) - 1);

    let number = value >> (LETTERS_BITS + SUFFIX_BITS);

    (number, suffix, letters)
}
