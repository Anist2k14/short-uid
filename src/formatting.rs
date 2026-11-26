use super::ShortUID;
use super::encoding;
use std::fmt;

impl fmt::Display for ShortUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (number, suffix, mut letters) = encoding::decode(self.as_u64());

        let mut chars = ['a'; 5];

        for i in (0..5).rev() {
            chars[i] = (b'a' + (letters % 26) as u8) as char;
            letters /= 26;
        }

        let n = format!("{number:09}");
        let s = format!("{suffix:02}");

        write!(
            f,
            "{}{}-{}{}-{}{}",
            &n[0..2],
            String::from_iter(&chars[0..3]),
            &n[2..6],
            String::from_iter(&chars[3..5]),
            &n[6..9],
            s,
        )
    }
}
