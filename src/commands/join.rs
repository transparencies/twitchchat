use crate::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

/// Join a channel
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Join<'a> {
    pub(crate) channel: &'a str,
}

/// Join a channel
pub const fn join(channel: &str) -> Join<'_> {
    Join { channel }
}

impl<'a> Encodable for Join<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).parts(&[&"JOIN", &self.channel])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn join_encode() {
        test_encode(join("#museun"), "JOIN #museun\r\n");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn join_serde() {
        test_serde(join("#museun"), "JOIN #museun\r\n");
    }
}