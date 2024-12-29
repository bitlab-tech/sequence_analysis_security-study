use std::fmt::{self, Binary, Debug, Error, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Shl, Shr};

#[derive(PartialEq, Eq)]
pub enum KmerType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128)
}

impl Shl<usize> for KmerType {
    type Output = KmerType;

    fn shl(self, rhs: usize) -> KmerType {
        match self {
            KmerType::U8(val) => KmerType::U8(val << rhs),
            KmerType::U16(val) => KmerType::U16(val << rhs),
            KmerType::U32(val) => KmerType::U32(val << rhs),
            KmerType::U64(val) => KmerType::U64(val << rhs),
            KmerType::U128(val) => KmerType::U128(val << rhs),
        }
    }
}

impl Shr<usize> for KmerType {
    type Output = KmerType;

    fn shr(self, rhs: usize) -> KmerType {
        match self {
            KmerType::U8(val) => KmerType::U8(val >> rhs),
            KmerType::U16(val) => KmerType::U16(val >> rhs),
            KmerType::U32(val) => KmerType::U32(val >> rhs),
            KmerType::U64(val) => KmerType::U64(val >> rhs),
            KmerType::U128(val) => KmerType::U128(val >> rhs),
        }
    }
}

impl BitOr for KmerType {
    type Output = KmerType;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => KmerType::U8(a | b),
            (KmerType::U16(a), KmerType::U16(b)) => KmerType::U16(a | b),
            (KmerType::U32(a), KmerType::U32(b)) => KmerType::U32(a | b),
            (KmerType::U64(a), KmerType::U64(b)) => KmerType::U64(a | b),
            (KmerType::U128(a), KmerType::U128(b)) => KmerType::U128(a | b),
            _ => panic!("Bitwise OR is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitAnd for KmerType {
    type Output = KmerType;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => KmerType::U8(a & b),
            (KmerType::U16(a), KmerType::U16(b)) => KmerType::U16(a & b),
            (KmerType::U32(a), KmerType::U32(b)) => KmerType::U32(a & b),
            (KmerType::U64(a), KmerType::U64(b)) => KmerType::U64(a & b),
            (KmerType::U128(a), KmerType::U128(b)) => KmerType::U128(a & b),
            _ => panic!("Bitwise AND is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitOrAssign<KmerType> for KmerType {
    fn bitor_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a |= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a |= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a |= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a |= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a |= b,
            _ => panic!("Bitwise OR is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitAndAssign<KmerType> for KmerType {
    fn bitand_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a &= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a &= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a &= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a &= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a &= b,
            _ => panic!("Bitwise AND is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitXorAssign for KmerType {
    fn bitxor_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a ^= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a ^= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a ^= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a ^= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a ^= b,
            _ => panic!("Bitwise XOR is not supported for mismatched KmerType variants"),
        }
    }
}

impl Debug for KmerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::U8(arg0) => f.debug_tuple("U8").field(arg0).finish(),
            Self::U16(arg0) => f.debug_tuple("U16").field(arg0).finish(),
            Self::U32(arg0) => f.debug_tuple("U32").field(arg0).finish(),
            Self::U64(arg0) => f.debug_tuple("U64").field(arg0).finish(),
            Self::U128(arg0) => f.debug_tuple("U128").field(arg0).finish(),
        }
    }
}

impl Binary for KmerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::U8(val) => fmt::Binary::fmt(&val, f),
            Self::U16(val) => fmt::Binary::fmt(&val, f),
            Self::U32(val) => fmt::Binary::fmt(&val, f),
            Self::U64(val) => fmt::Binary::fmt(&val, f),
            Self::U128(val) => fmt::Binary::fmt(&val, f),
        }
    }
}

pub trait ToUInt {
    // fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    // fn to_u32(&self) -> u32;
    // fn to_u64(&self) -> u64;
    // fn to_u128(&self) -> u128;
}

impl ToUInt for KmerType {
    // fn to_u8(&self) -> u8 {
    //     match self {
    //         Self::U8(val) => *val,
    //         Self::U16(_) => panic!("U16 is not supported"),
    //         Self::U32(_) => panic!("U32 is not supported"),
    //         Self::U64(_) => panic!("U64 is not supported"),
    //         Self::U128(_) => panic!("U128 is not supported"),
    //     }
    // }

    fn to_u16(&self) -> u16 {
        match self {
            Self::U8(_) => panic!("U8 is not supported"),
            Self::U16(val) => *val,
            Self::U32(_) => panic!("U32 is not supported"),
            Self::U64(_) => panic!("U64 is not supported"),
            Self::U128(_) => panic!("U128 is not supported"),
        }
    }
}

pub fn init_kmer_type(len: usize, val: u8) -> KmerType {
    if len <= 4 {
        KmerType::U8(val)
    } else if len <= 8 {
        KmerType::U16(val.into())
    } else if len <= 16 {
        KmerType::U32(val.into())
    } else if len <= 32 {
        KmerType::U64(val.into())
    } else if len <= 64 {
        KmerType::U128(val.into())
    } else {
        panic!("kmer length not supported");
    }
}

pub fn binary_encode(kmer: &str) -> KmerType {
    let kmer_len: usize = kmer.len();

    let mut result: KmerType = init_kmer_type(kmer_len, 0b00);

    for (i, c) in kmer.chars().enumerate() {
        let val: KmerType = match c {
            'A' => init_kmer_type(kmer_len, 0b00),
            'C' => init_kmer_type(kmer_len, 0b01),
            'G' => init_kmer_type(kmer_len, 0b10),
            'T' => init_kmer_type(kmer_len, 0b11),
            _ => panic!("Invalid character in kmer"),
        };
        result |= val << (kmer_len - 1 - i) * 2;
    }
    result
}