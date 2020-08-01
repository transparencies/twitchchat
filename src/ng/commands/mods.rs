use crate::ng::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Mods<'a> {
    pub channel: &'a str,
}

pub fn mods(channel: &str) -> Mods<'_> {
    Mods { channel }
}

impl<'a> Encodable for Mods<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(self.channel, &[&"/mods"])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn mods_encode() {
        test_encode(mods("#museun"), "PRIVMSG #museun :/mods\r\n")
    }

    #[test]
    #[cfg(feature = "serde")]
    fn mods_serde() {
        test_serde(mods("#museun"), "PRIVMSG #museun :/mods\r\n")
    }
}
