use core::cmp::Ordering;
use std::fmt;

/// Little-endian large integer type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Uint256(pub [u64; 4]);

impl Uint256 {
    #[inline(always)]
    pub fn new(v: [u64; 4]) -> Self {
        Self(v)
    }
    /// Create an object from a given unsigned 64-bit integer
    #[inline]
    pub fn from_u64(init: u64) -> Uint256 {
        let mut ret = [0; 4];
        ret[0] = init;
        Uint256(ret)
    }

    /// Creates big integer value from a byte slice using
    /// little-endian encoding
    #[inline(always)]
    pub fn from_le_bytes(bytes: [u8; 32]) -> Uint256 {
        let mut out = [0u64; 4];
        // This should optimize to basically a transmute.
        out.iter_mut()
            .zip(bytes.chunks_exact(8))
            .for_each(|(word, bytes)| *word = u64::from_le_bytes(bytes.try_into().unwrap()));
        Self(out)
    }

    #[inline(always)]
    pub fn to_le_bytes(self) -> [u8; 32] {
        let mut out = [0u8; 32];
        // This should optimize to basically a transmute.
        out.chunks_exact_mut(8).zip(self.0).for_each(|(bytes, word)| bytes.copy_from_slice(&word.to_le_bytes()));
        out
    }

    /// Converts compact target bits (as used in Bitcoin/Turkium headers) to a Uint256 target
    /// Compact format: 3 bytes of mantissa + 1 byte of exponent
    #[inline]
    pub fn from_compact_target_bits(bits: u32) -> Self {
        let exponent = (bits >> 24) as u8;
        let mantissa = bits & 0x00ffffff;

        if exponent <= 3 {
            // Shift right
            Uint256::from_u64((mantissa >> (8 * (3 - exponent))) as u64)
        } else {
            // Shift left
            let mut result = [0u64; 4];
            let shift_bytes = exponent as usize - 3;
            let shift_bits = shift_bytes * 8;
            let word_shift = shift_bits / 64;
            let bit_shift = shift_bits % 64;

            if word_shift < 4 {
                result[word_shift] = (mantissa as u64) << bit_shift;
                if bit_shift > 0 && word_shift + 1 < 4 {
                    result[word_shift + 1] = (mantissa as u64) >> (64 - bit_shift);
                }
            }
            Uint256(result)
        }
    }
}

impl fmt::LowerHex for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_le_bytes().iter().try_for_each(|&c| write!(f, "{:02x}", c))
    }
}

impl PartialOrd for Uint256 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Uint256) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Uint256 {
    #[inline(always)]
    fn cmp(&self, other: &Uint256) -> Ordering {
        // We need to manually implement ordering because we use little-endian
        // and the auto derive is a lexicographic ordering(i.e. memcmp)
        // which with numbers is equivalent to big-endian
        Iterator::cmp(self.0.iter().rev(), other.0.iter().rev())
    }
}

impl core::ops::Shl<usize> for Uint256 {
    type Output = Uint256;

    fn shl(self, shift: usize) -> Uint256 {
        let Uint256(ref original) = self;
        let mut ret = [0u64; 4];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;
        for i in 0..4 {
            // Shift
            if bit_shift < 64 && i + word_shift < 4 {
                ret[i + word_shift] += original[i] << bit_shift;
            }
            // Carry
            if bit_shift > 0 && i + word_shift + 1 < 4 {
                ret[i + word_shift + 1] += original[i] >> (64 - bit_shift);
            }
        }
        Uint256(ret)
    }
}
