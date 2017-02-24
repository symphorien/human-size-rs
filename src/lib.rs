#![warn(missing_docs)]
#![feature(i128_type)]
#![feature(try_from)]

//! The `human_size` represents sizes for humans. The main type is [`Size`],
//! which (as the name might suggests) represents a size. See the example below
//! to get an idea of what [`Size`] is capable of.
//!
//! ```
//! # #![feature(try_from)]
//! # use human_size::{Size, Multiple};
//! # use std::convert::TryInto;
//! let my_size: Size = "1000 B".parse().expect("unable to parse size");
//! let same_size = Size::new(1, Multiple::Kilobyte);
//! assert_eq!(my_size, same_size);
//!
//! println!("The size is {}", my_size); // The size is 1000 B
//! println!("Or {}", same_size); // Or 1 kB
//! let in_int: u64 = my_size.try_into().unwrap();
//! println!("Or even {}", in_int); // Or even 1000
//! ```
//!
//! [`Size`]: struct.Size.html

use std::convert::TryInto;
use std::str::FromStr;
use std::cmp::{PartialOrd, Ordering};
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

#[cfg(test)]
mod tests;

/// `Size` represent a size of something... for example a file.
///
/// `Size` supports a lot of common operations like parsing a size from a string,
/// by implementing the [`FromStr`] trait.
///
/// It can also be converted into an integer, which returns the result in number
/// of bytes, this is done by implementing the [`TryInto`] trait for several
/// sized integers. To convert the size into a string the [`Display`] trait is
/// implemented.
///
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
/// [`TryInto`]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html
/// [`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
#[derive(Debug, Clone)]
pub struct Size {
    value: u32,
    multiple: Multiple,
}

impl Size {
    /// Create a new size. If the value can't be representated in [`u32`], use a
    /// bigger [`Multiple`].
    ///
    /// [`u32`]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
    /// [`Multiple`]: enum.Multiple.html
    pub fn new(value: u32, multiple: Multiple) -> Size {
        Size{
            value: value,
            multiple: multiple,
        }
    }
}

impl TryInto<u32> for Size {
    type Err = ConversionError;

    /// Converts the `Size` into a unsigned 32 bit integer. Due to the limited
    /// number of bits in `u32`, any `Size` with a [`Multiple`] bigger then
    /// [`Multiple::Gigabyte`][] (10^9) or [`Multiple::Gigibyte`][] (2^30) can
    /// **not** be converted into an `u32` and returns an error.
    ///
    /// [`Multiple`]: enum.Multiple.html
    /// [`Multiple::Gigabyte`]: enum.Multiple.html#variant.Gigabyte
    /// [`Multiple::Gigibyte`]: enum.Multiple.html#variant.Gigibyte
    fn try_into(self) -> Result<u32, ConversionError> {
        let multiple: u32 = self.multiple.try_into()?;
        self.value.checked_mul(multiple).ok_or(ConversionError::Overflow)
    }
}

impl TryInto<u64> for Size {
    type Err = ConversionError;

    /// Converts the `Size` into a unsigned 64 bit integer. Due to the limited
    /// number of bits in `u64`, any `Size` with a [`Multiple`] bigger then
    /// [`Multiple::Petabyte`][] (10^15) or [`Multiple::Pebibyte`][] (2^50) can
    /// **not** be converted into an `u64` and returns an error.
    ///
    /// [`Multiple`]: enum.Multiple.html
    /// [`Multiple::Petabyte`]: enum.Multiple.html#variant.Petabyte
    /// [`Multiple::Pebibyte`]: enum.Multiple.html#variant.Pebibyte
    fn try_into(self) -> Result<u64, ConversionError> {
        let multiple: u64 = self.multiple.try_into()?;
        (self.value as u64).checked_mul(multiple).ok_or(ConversionError::Overflow)
    }
}

impl TryInto<u128> for Size {
    type Err = ConversionError;

    /// Converts the `Size` into a unsigned 64 bit integer. Due to the limited
    /// number of bits in `u128` it will return an error if the value overflows.
    fn try_into(self) -> Result<u128, ConversionError> {
        let multiple: u128 = self.multiple.try_into()?;
        (self.value as u128).checked_mul(multiple).ok_or(ConversionError::Overflow)
    }
}

impl FromStr for Size {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Size, Self::Err> {
        let mut parts = input.split_whitespace();
        let value = parts.next().ok_or(ParsingError::NoValue)?
            .parse().or_else(|err| Err(ParsingError::InvalidValue(err)))?;
        let multiple = parts.next().ok_or(ParsingError::NoMultiple)?
            .parse()?;

        if parts.next().is_some() {
            Err(ParsingError::UnknownExtra)
        } else {
            Ok(Size::new(value, multiple))
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        self.partial_cmp(other)
            .and_then(|order| Some(order == Ordering::Equal))
            .unwrap_or(false)
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        (*self).clone().try_into().ok().and_then(|left: u128| {
            (*other).clone().try_into().ok().and_then(|right: u128| left.partial_cmp(&right))
        })
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.multiple)
    }
}

/// A `Multiple` represent a multiple of bytes. This is mainly used to keep track
/// of what multiple [`Size`] uses, so it can display it using the same multiple
/// of bytes.
///
/// `Multiple` supports a lot of common operations like parsing a multiple from
/// a string, by implementing the [`FromStr`] trait. As well as converting into
/// an integer, which returns the number of bytes the multiple represents (e.g.
/// `1.000` for [`Kilobyte`]), by implementing the [`TryInto`] trait for several
/// sized integers. To convert the size into a string the [`Display`] trait is
/// implemented.
///
/// [`Byte`]: enum.Multiple.html#variant.Byte
/// [`Kilobyte`]: enum.Multiple.html#variant.Kilobyte
/// [`Kibibyte`]: enum.Multiple.html#variant.Kibibyte
/// [`Size`]: struct.Size.html
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
/// [`TryInto`]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html
/// [`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Multiple {
    /// Represents a single byte, value * 1, "B" when parsing text.
    Byte,


    /// A kilobyte, value * 1,000 (1000^1), "kB" in when parsing from text.
    Kilobyte,

    /// A megabyte, value * 1,000,000 (1000^2), "MB" in when parsing from text.
    Megabyte,

    /// A gigabyte, value * 1,000,000,000 (1000^3), "GB" in when parsing from
    /// text.
    Gigabyte,

    /// A terabyte, value * 1,000,000,000,000 (1000^4), "TB" in when parsing
    /// from text.
    Terabyte,

    /// A petabyte, value * 1,000,000,000,000,000 (1000^5), "PB" in when
    /// parsing from text.
    Petabyte,

    /// A exabyte, value * 1,000,000,000,000,000,000 (1000^6), "EB" in when
    /// parsing from text.
    Exabyte,

    /// A zettabyte, value * 1,000,000,000,000,000,000,000 (1000^7), "ZB" in
    /// when parsing from text.
    Zettabyte,

    /// A yottabyte, value * 1,000,000,000,000,000,000,000,000 (1000^8), "YB"
    /// in when parsing from text.
    Yottabyte,


    /// A kibibyte, value * 1,024 (1024^1), "KiB", or "KB" in when parsing from
    /// text.
    Kibibyte,

    /// A mebibyte, value * 1,048,576 (1024^2), "MiB" in when parsing from text.
    Mebibyte,

    /// A gigibyte, value * 1,073,741,824 (1024^3), "GiB" in when parsing from
    /// text.
    Gigibyte,

    /// A tebibyte, value * 1,099,511,627,776 (1024^4), "TiB" in when parsing
    /// from text.
    Tebibyte,

    /// A pebibyte, value * 1,125,899,906,842,624 (1024^5), "PiB" in when
    /// parsing from text.
    Pebibyte,

    /// A exbibyte, value * 1,152,921,504,606,846,976 (1024^6), "EiB" in when
    /// parsing from text.
    Exbibyte,

    /// A zebibyte, value * 1,180,591,620,717,411,303,424 (1024^7), "ZiB" in
    /// when parsing from text.
    Zebibyte,

    /// A yobibyte, value * 1,208,925,819,614,629,174,706,176 (1024^8), "YiB"
    /// in when parsing from text.
    Yobibyte,
}

impl TryInto<u32> for Multiple {
    type Err = ConversionError;

    /// Converts the `Multiple` into a unsigned 32 bit integer. Due to the limited
    /// number of bits in `u32`, anything bigger then [`Multiple::Gigabyte`][]
    /// (10^9) or [`Multiple::Gigibyte`][] (2^30) can **not** be converted into
    /// an `u32` and will return an error.
    ///
    /// [`Multiple::Gigabyte`]: enum.Multiple.html#variant.Gigabyte
    /// [`Multiple::Gigibyte`]: enum.Multiple.html#variant.Gigibyte
    fn try_into(self) -> Result<u32, Self::Err> {
        match self {
            Multiple::Byte => Ok(1),

            Multiple::Kilobyte => Ok(1_000),
            Multiple::Megabyte => Ok(1_000_000),
            Multiple::Gigabyte => Ok(1_000_000_000),

            Multiple::Kibibyte => Ok(1024),
            Multiple::Mebibyte => Ok(1_048_576),
            Multiple::Gigibyte => Ok(1_073_741_824),

            _ => Err(ConversionError::Overflow),
        }
    }
}

impl TryInto<u64> for Multiple {
    type Err = ConversionError;

    /// Converts the `Multiple` into a unsigned 64 bit integer. Due to the limited
    /// number of bits in `u64`, anything bigger then [`Multiple::Petabyte`][]
    /// (10^15) or [`Multiple::Pebibyte`][] (2^50) can **not** be converted into
    /// an `u64` and will return an error.
    ///
    /// [`Multiple::Petabyte`]: enum.Multiple.html#variant.Petabyte
    /// [`Multiple::Pebibyte`]: enum.Multiple.html#variant.Pebibyte
    fn try_into(self) -> Result<u64, Self::Err> {
        match self {
            Multiple::Terabyte => Ok(1_000_000_000_000),
            Multiple::Petabyte => Ok(1_000_000_000_000_000),

            Multiple::Tebibyte => Ok(1_099_511_627_776),
            Multiple::Pebibyte => Ok(1_125_899_906_842_624),

            _ => self.try_into().map(|value: u32| value as u64)
        }
    }
}

impl TryInto<u128> for Multiple {
    type Err = ConversionError;

    /// Converts the `Multiple` into a unsigned 128 bit integer.
    fn try_into(self) -> Result<u128, Self::Err> {
        match self {
            Multiple::Exabyte => Ok(1_000_000_000_000_000_000),
            Multiple::Zettabyte => Ok(1_000_000_000_000_000_000_000),
            Multiple::Yottabyte => Ok(1_000_000_000_000_000_000_000_000),

            Multiple::Exbibyte => Ok(1_152_921_504_606_846_976),
            Multiple::Zebibyte => Ok(1_180_591_620_717_411_303_424),
            Multiple::Yobibyte => Ok(1_208_925_819_614_629_174_706_176),

            _ => self.try_into().map(|value: u64| value as u128)
        }
    }
}

impl FromStr for Multiple {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Multiple, Self::Err> {
        match input {
            "B" => Ok(Multiple::Byte),

            "kB" => Ok(Multiple::Kilobyte),
            "MB" => Ok(Multiple::Megabyte),
            "GB" => Ok(Multiple::Gigabyte),
            "TB" => Ok(Multiple::Terabyte),
            "PB" => Ok(Multiple::Petabyte),
            "EB" => Ok(Multiple::Exabyte),
            "ZB" => Ok(Multiple::Zettabyte),
            "YB" => Ok(Multiple::Yottabyte),

            "KB" => Ok(Multiple::Kibibyte),
            "KiB" => Ok(Multiple::Kibibyte),
            "MiB" => Ok(Multiple::Mebibyte),
            "GiB" => Ok(Multiple::Gigibyte),
            "TiB" => Ok(Multiple::Tebibyte),
            "PiB" => Ok(Multiple::Pebibyte),
            "EiB" => Ok(Multiple::Exbibyte),
            "ZiB" => Ok(Multiple::Zebibyte),
            "YiB" => Ok(Multiple::Yobibyte),

            _ => Err(ParsingError::UnknownMultiple),
        }
    }
}

impl fmt::Display for Multiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Multiple::Byte => "B",

            Multiple::Kilobyte => "kB",
            Multiple::Megabyte => "MB",
            Multiple::Gigabyte => "GB",
            Multiple::Terabyte => "TB",
            Multiple::Petabyte => "PB",
            Multiple::Exabyte => "EB",
            Multiple::Zettabyte => "ZB",
            Multiple::Yottabyte => "YB",

            Multiple::Kibibyte => "KiB",
            Multiple::Mebibyte => "MiB",
            Multiple::Gigibyte => "GiB",
            Multiple::Tebibyte => "TiB",
            Multiple::Pebibyte => "PiB",
            Multiple::Exbibyte => "EiB",
            Multiple::Zebibyte => "ZiB",
            Multiple::Yobibyte => "YiB",
        };
        write!(f, "{}", value)
    }
}

/// The error returned when trying to parse a [`Size`] or [`Mulitple`] from a
/// string, using the [`FromStr`] trait.
///
/// [`Size`]: struct.Size.html
/// [`Mulitple`]: enum.Multiple.html
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParsingError {
    /// The provided string is missing a value.
    NoValue,

    /// The value is invalid and failed to be parsed.
    InvalidValue(ParseIntError),

    /// The value is missing the multiple.
    NoMultiple,

    /// The multiple in the string is unknown.
    UnknownMultiple,

    /// Extra unknown data was provided.
    UnknownExtra,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::NoValue => "no value",
            ParsingError::InvalidValue(_) => "invalid value",
            ParsingError::NoMultiple => "no multiple",
            ParsingError::UnknownMultiple => "unknown multiple",
            ParsingError::UnknownExtra => "unknown extra data",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParsingError::InvalidValue(ref cause) => Some(cause),
            _ => None,
        }
    }
}

/// The error returned when trying to convert a [`Size`] or [`Mulitple`] into
/// another value, using the [`TryInto`] trait.
///
/// [`Size`]: struct.Size.html
/// [`Mulitple`]: enum.Multiple.htmlA
/// [`TryInto`]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConversionError {
    /// The value could not be converted into an integer, because it overflows
    /// the integer. Try a bigger integer.
    Overflow,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ConversionError {
    fn description(&self) -> &str {
        match *self {
            ConversionError::Overflow => "size overflows integer",
        }
    }
}
