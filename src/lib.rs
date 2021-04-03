//! # fmtsize
//!
//! `fmtsize` provides human-readable formatting for things like file sizes. It
//! attempts to find the largest shorthand size possible for a given value,
//! although it's limited to "gigabytes." Someday we may upgrade to terabytes. :)
//! 
//! ```
//! # use fmtsize::{Conventional, FmtSize};
//! println!("{}", 492_752_310_u64.fmt_size(Conventional)); // 469.93 MB
//! ```

use std::fmt::{self, Display};

mod conventional {
    pub const KILOBYTE: u64 = 1 << 10;
    pub const MEGABYTE: u64 = 1 << 20;
    pub const GIGABYTE: u64 = 1 << 30;
}

mod decimal {
    pub const KILOBYTE: u64 = 1000;
    pub const MEGABYTE: u64 = 1_000_000;
    pub const GIGABYTE: u64 = 1_000_000_000;
}

/// Used to format values in accordance with
/// some set of named sizes and constants.
pub trait Format {
    /// The appropriate divisor for a given value size.
    ///
    /// E.g., to derive the number of megabytes in a given file, divide
    /// file size by the size in bytes of one megabyte.
    fn divisor(&self, size: u64) -> u64;

    /// The appropriate name for a given value size.
    ///
    /// For instance, something larger than a single megabyte and smaller than
    /// one gigabyte will be called "megabytes."
    fn name(&self, size: u64) -> &'static str;
}

/// Old-school formatting: a megabyte is 1024 kilobytes, dammit!
#[derive(Copy, Clone, Debug, Default)]
pub struct Conventional;

impl Format for Conventional {
    fn divisor(&self, size: u64) -> u64 {
        use conventional::*;
        match size {
            size if size < MEGABYTE => KILOBYTE,
            size if size < GIGABYTE => MEGABYTE,
            _ => GIGABYTE,
        }
    }

    fn name(&self, size: u64) -> &'static str {
        use conventional::*;
        match size {
            size if size < MEGABYTE => "KB",
            size if size < GIGABYTE => "MB",
            _ => "GB",
        }
    }
}

/// Nonsense formatting. "That hard drive is totally 1 TB! You're
/// thinking of a TiB, which is totally different...."
#[derive(Copy, Clone, Debug, Default)]
pub struct Decimal;

impl Format for Decimal {
    fn divisor(&self, size: u64) -> u64 {
        use decimal::*;
        match size {
            size if size < MEGABYTE => KILOBYTE,
            size if size < GIGABYTE => MEGABYTE,
            _ => GIGABYTE,
        }
    }

    fn name(&self, size: u64) -> &'static str {
        use conventional::*;
        match size {
            size if size < MEGABYTE => "KB",
            size if size < GIGABYTE => "MB",
            _ => "GB",
        }
    }
}

/// Lazy memory size formatter.
pub struct ByteSizeFormatter<F = Conventional> {
    size: u64,
    fmt: F,
}

impl<F: Format> Display for ByteSizeFormatter<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let divisor = self.fmt.divisor(self.size) as f32;
        let size = self.size as f32 / divisor;
        write!(f, "{:.2} {}", size, self.fmt.name(self.size))
    }
}

pub trait FmtSize {
    /// Format a memory size value according to a given format provider.
    ///
    /// The formatter resulting from this call is lazy.
    fn fmt_size<F: Format>(self, fmt: F) -> ByteSizeFormatter<F>;
}

impl FmtSize for u64 {
    fn fmt_size<F: Format>(self, fmt: F) -> ByteSizeFormatter<F> {
        ByteSizeFormatter { size: self, fmt }
    }
}

#[cfg(test)]
mod tests {
    use super::{Conventional, FmtSize};

    #[test]
    fn it_works() {
        let expected = "1.00 MB";
        let actual = 1_048_576.fmt_size(Conventional).to_string();
        assert_eq!(expected, actual);
    }
}
