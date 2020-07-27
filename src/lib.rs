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

pub trait Format {
    fn divisor(&self, size: u64) -> u64;
    fn name(&self, size: u64) -> &'static str;
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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
