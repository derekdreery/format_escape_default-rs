//! A library for printing (formatting) arbitrary byte arrays using [`std::ascii::escape_default`].
//!
//! [`std::ascii::escape_default`]: https://doc.rust-lang.org/std/ascii/fn.escape_default.html
// #![no_std] // can't do this because ascii isn't in core
use std::fmt;

/// Wraps the byte array and implements display
///
/// See [`format_escape_default`] method.
///
/// [`format_escape_default`]: fn.format_escape_default.html
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct EscapeDefaultFmt<'a> {
    inner: &'a [u8],
}

impl<'a> fmt::Display for EscapeDefaultFmt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::ascii;
        use std::char;
        for ch in self.inner {
            for esc_ch in ascii::escape_default(*ch) {
                // safe because escape_default guarantees a valid unicode character
                write!(f, "{}", unsafe { char::from_u32_unchecked(esc_ch as u32) })?;
            }
        }
        Ok(())
    }
}

/// Takes a byte array and wraps it in a struct that knows how to format it using
/// [`std::ascii::escape_default`].
///
/// # Examples
///
/// ```
/// use format_escape_default::format_escape_default;
///
/// let unescaped = b"\t\nsometext\r\n";
/// let escaped = "\\t\\nsometext\\r\\n";
/// assert_eq!(escaped, format!("{}", format_escape_default(unescaped)));
/// ```
///
/// [`std::ascii::escape_default`]: https://doc.rust-lang.org/std/ascii/fn.escape_default.html
pub fn format_escape_default<'a>(i: &'a [u8]) -> EscapeDefaultFmt<'a> {
    EscapeDefaultFmt { inner: i }
}

#[cfg(test)]
mod tests {
    use super::format_escape_default;
    #[test]
    fn smoke() {
        assert_eq!(format!("{}", format_escape_default(b"\t\r\ntest\\")), "\\t\\r\\ntest\\\\");
    }
}
