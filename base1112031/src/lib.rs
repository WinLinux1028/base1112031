#![feature(unchecked_math)]

pub(crate) mod convert;
use num_bigint::BigUint;

use crate::convert::{FromReverseVecChar, ToVecChar};

use std::ops::{DivAssign, Rem};

/// # Examples
/// ```
/// use use base1112031::ToBase1112031;
///
/// let input = 139449924812_u64;
/// let result: String = input.to_base1112031().unwrap();
/// assert_eq!(result, "🇺🇦");
/// ```
pub trait ToBase1112031:
    Clone + TryInto<u32> + TryFrom<u32> + for<'a> DivAssign<&'a Self> + PartialEq<Self>
where
    for<'a> &'a Self: Rem<&'a Self, Output = Self>,
{
    /// # Errors
    /// - When the input could not be converted to u32.
    fn to_base1112031<T>(mut self) -> Option<T>
    where
        T: FromReverseVecChar,
    {
        let zero: Self = 0_u32.try_into().ok()?;
        let base: Self = 1112031_u32.try_into().ok()?;
        let mut result: Vec<char> = Vec::new(); // 下桁から順になる
        loop {
            // 1112031進数の桁を1つ生成する
            let digit: u32 = (&self % &base).try_into().ok()?;

            // 桁をそれぞれコードポイントに変換する
            let convert: i32 = match digit {
                0..=9 => 0x30,             // 0 to 9
                10..=35 => 0x61 - 10,      // a to z
                36..=61 => 0x41 - 36,      // A to Z
                62..=77 => 0x20 - 62,      // 空白 to /
                78..=84 => 0x3A - 78,      // : to @
                85..=90 => 0x5B - 85,      // [ to `
                91..=94 => 0x7B - 91,      // { to ~
                95..=55262 => 0x80 - 95,   // U+0080 to U+D7FF
                55263.. => 0xE000 - 55263, // U+E000 to U+10FFFF
            };
            // u32とi32を足すためにunsafeが必要
            unsafe {
                let convert: u32 = std::mem::transmute(convert);
                result.push(char::from_u32_unchecked(digit.unchecked_add(convert)));
            }

            self /= &base;

            if self == zero {
                break;
            }
        }

        Some(FromReverseVecChar::from(result))
    }
}
impl<T> ToBase1112031 for T
where
    T: Clone + TryInto<u32> + TryFrom<u32> + for<'a> DivAssign<&'a Self> + PartialEq<Self>,
    for<'a> &'a Self: Rem<&'a Self, Output = Self>,
{
}

/// # Examples
/// ```
/// use use base1112031::FromBase1112031;
///
/// let input = "🇺🇦";
/// let result: u64 = FromBase1112031::from_base1112031(input).unwrap();
/// assert_eq!(result, 139449924812);
/// ```
pub trait FromBase1112031: TryFrom<BigUint> {
    /// # Errors
    /// - When the type specified for the output would overflow.
    fn from_base1112031<T: ToVecChar>(input: T) -> Option<Self> {
        let input = input
            .to_vec_char()
            .into_iter()
            .rev() // 下桁から順に並び替える
            .map(|i| i as u32); // コードポイントに変換

        let mut exp = BigUint::from(1_u8);
        let mut result = BigUint::from(0_u8);
        for mut i in input {
            // コードポイントをそれぞれ対応する値に変換
            let convert: i32 = match i {
                0x30..=0x39 => 0x30,                 // 0 to 9
                0x61..=0x7A => 0x61 - 10,            // 10 to 35
                0x41..=0x5A => 0x41 - 36,            // 36 to 61
                0x20..=0x2F => 0x20 - 62,            // 62 to 77
                0x3A..=0x40 => 0x3A - 78,            // 78 to 84
                0x5B..=0x60 => 0x5B - 85,            // 85 to 90
                0x7B..=0x7E => 0x7B - 91,            // 91 to 94
                0x80..=0xD7FF => 0x80 - 95,          // 95 to 55262
                0xE000..=0x10FFFF => 0xE000 - 55263, // 55263 to 1112030
                _ => return None,
            };
            // u32からi32を引くためにunsafeが必要
            unsafe {
                let convert: u32 = std::mem::transmute(convert);
                i = i.unchecked_sub(convert);
            }

            // i * 1112031^nをresultに足す
            result += &exp * i;

            // 次のループのための処理
            exp *= 1112031_u32;
        }

        result.try_into().ok()
    }
}
impl<T> FromBase1112031 for T where T: TryFrom<BigUint> {}
