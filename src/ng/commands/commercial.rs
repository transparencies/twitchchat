use crate::ng::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Commercial<'a> {
    pub(crate) channel: &'a str,
    pub(crate) length: Option<usize>,
}

impl<'a> Commercial<'a> {
    pub fn new(channel: &'a str, length: impl Into<Option<usize>>) -> Self {
        Self {
            channel,
            length: length.into(),
        }
    }
}

pub fn commercial(channel: &str, length: impl Into<Option<usize>>) -> Commercial<'_> {
    Commercial::new(channel, length)
}

impl<'a> Encodable for Commercial<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(
            &self.channel,
            &[
                &"/commercial",
                &self
                    .length
                    .map(|s| s.to_string())
                    .as_deref()
                    .unwrap_or_default(),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn commercial_encode() {
        test_encode(
            commercial("#museun", None),
            "PRIVMSG #museun :/commercial\r\n",
        );
        test_encode(
            commercial("#museun", 10),
            "PRIVMSG #museun :/commercial 10\r\n",
        );
        test_encode(
            commercial("#museun", Some(10)),
            "PRIVMSG #museun :/commercial 10\r\n",
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn commercial_serde() {
        test_serde(
            commercial("#museun", None),
            "PRIVMSG #museun :/commercial\r\n",
        );
        test_serde(
            commercial("#museun", 10),
            "PRIVMSG #museun :/commercial 10\r\n",
        );
        test_serde(
            commercial("#museun", Some(10)),
            "PRIVMSG #museun :/commercial 10\r\n",
        );
    }
}