use crate::{FromIrcMessage, InvalidMessage, Validator};
use crate::{IrcMessage, Str, StrIndex};

/// A ping request from the server
#[derive(Debug, Clone, PartialEq)]
pub struct Ping<'t> {
    raw: Str<'t>,
    token: StrIndex,
}

impl<'t> Ping<'t> {
    raw!();
    str_field!(
        /// Token associated with the PING event
        token
    );
}

impl<'t> FromIrcMessage<'t> for Ping<'t> {
    type Error = InvalidMessage;

    fn from_irc(msg: IrcMessage<'t>) -> Result<Self, Self::Error> {
        msg.expect_command(IrcMessage::PING)?;

        let this = Self {
            token: msg.expect_data_index()?,
            raw: msg.raw,
        };

        Ok(this)
    }
}

into_owned!(Ping { raw, token });

serde_struct!(Ping { raw, token });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::irc;

    #[test]
    #[cfg(feature = "serde")]
    fn ping_serde() {
        let input = "PING :1234567890\r\n";
        crate::serde::round_trip_json::<Ping>(input);
    }

    #[test]
    fn ping() {
        let input = "PING :1234567890\r\n";
        for msg in irc::parse(input).map(|s| s.unwrap()) {
            let msg = Ping::from_irc(msg).unwrap();
            assert_eq!(msg.token(), "1234567890");
        }
    }
}