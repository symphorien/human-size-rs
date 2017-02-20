use std::convert::Into;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Size {
    value: u64,
    multiple: Multiple,
}

impl Size {
    pub fn new(value: u64, multiple: Multiple) -> Size {
        Size{
            value: value,
            multiple: multiple,
        }
    }
}

// TODO(Thomas): replace this with the TryInto trait, since it can fail?
impl Into<u64> for Size {
    /// Converts the `Size` into a unsigned 64 bit integer.
    ///
    /// # Panics
    ///
    /// Due to the limited number of bits in `u64`, any Size with a
    /// [`Multiple`](enum.Multiple.html) bigger then
    /// [`Multiple::Petabyte`](#variant.Petabyte) (10^15) or
    /// [`Multiple::Pebibyte`](#variant.Pebibyte) (2^50) can **not** be converted
    /// into an `u64` and will panic.
    fn into(self) -> u64 {
        let multiple: u64 = self.multiple.into();
        self.value * multiple
    }
}

#[derive(Debug, Clone, Copy)]
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

// TODO(Thomas): replace this with the TryInto trait, since it can fail?
impl Into<u64> for Multiple {
    /// Converts the `Multiple` into a unsigned 64 bit integer.
    ///
    /// # Panics
    ///
    /// Due to the limited number of bits in `u64`, anything bigger then
    /// [`Multiple::Petabyte`](#variant.Petabyte) (10^15) or
    /// [`Multiple::Pebibyte`](#variant.Pebibyte) (2^50) can **not** be converted
    /// into an `u64` and will panic.
    fn into(self) -> u64 {
        match self {
            Multiple::Byte => 1,

            Multiple::Kilobyte => 1_000,
            Multiple::Megabyte => 1_000_000,
            Multiple::Gigabyte => 1_000_000_000,
            Multiple::Terabyte => 1_000_000_000_000,
            Multiple::Petabyte => 1_000_000_000_000_000,

            Multiple::Kibibyte => 1024,
            Multiple::Mebibyte => 1_048_576,
            Multiple::Gigibyte => 1_073_741_824,
            Multiple::Tebibyte => 1_099_511_627_776,
            Multiple::Pebibyte => 1_125_899_906_842_624,

            _ => panic!("tried to convert a Multiple bigger then Petabyte or Pebibyte to u64"),
        }
    }
}
