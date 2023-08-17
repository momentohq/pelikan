// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use super::*;
use std::io::{Error, ErrorKind};

#[metric(name = "ping2")]
pub static PING: Counter = Counter::new();

#[metric(name = "ping_ex")]
pub static PING_EX: Counter = Counter::new();

#[derive(Debug, PartialEq, Eq)]
pub struct Ping {}
impl TryFrom<Message> for Ping {
    type Error = Error;

    fn try_from(other: Message) -> Result<Self, Error> {
        if let Message::Array(array) = other {
            if array.inner.is_none() {
                return Err(Error::new(ErrorKind::Other, "malformed command"));
            }
            Ok(Self {})
        } else {
            Err(Error::new(ErrorKind::Other, "malformed command"))
        }
    }
}

impl Ping {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<&Ping> for Message {
    fn from(_: &Ping) -> Message {
        Message::Array(Array {
            inner: Some(vec![Message::BulkString(BulkString::new(b"Ping"))]),
        })
    }
}

impl Compose for Ping {
    fn compose(&self, buf: &mut dyn BufMut) -> usize {
        let message = Message::from(self);
        message.compose(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let parser = RequestParser::new();
        assert_eq!(
            parser.parse(b"PING\r\n").unwrap().into_inner(),
            Request::Ping(Ping::new())
        );
    }
}
