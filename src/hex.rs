use core::fmt::Write as _;
const ALPHABET: &[u8] = b"0123456789abcdef";

pub struct HexEncoder<'a> {
    input: &'a [u8],
}

impl core::fmt::Display for HexEncoder<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for byte in self.input {
            f.write_char(ALPHABET[(byte >> 4) as usize] as char)?;
            f.write_char(ALPHABET[(byte & 0xf) as usize] as char)?;
        }
        Ok(())
    }
}

// Only one type of formatting is valid.
// But we support Debug just for convenience.
impl core::fmt::Debug for HexEncoder<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

pub fn encode(input: &[u8]) -> HexEncoder<'_> {
    HexEncoder { input }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let data = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        // Note: Display allows us to use to_string() without us using std or alloc.
        let encoded = encode(&data).to_string();
        assert_eq!(encoded, "123456789abcdef0");
    }
}
