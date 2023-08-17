// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::Error;
use libc::socket;
use momento::SimpleCacheClient;
use net::TCP_SEND_BYTE;
use protocol_resp::Ping;
use session::{SESSION_SEND, SESSION_SEND_BYTE, SESSION_SEND_EX};
use tokio::io::AsyncWriteExt;

const PONG_RSP: &[u8; 7] = b"+PONG\r\n";

pub async fn ping(
    client: &mut SimpleCacheClient,
    cache_name: &str,
    response_buf: &mut Vec<u8>,
    req: &Ping,
) -> Result<(), Error> {
    response_buf.extend_from_slice(PONG_RSP);
    SESSION_SEND.increment();
    SESSION_SEND_BYTE.add(response_buf.len() as _);
    TCP_SEND_BYTE.add(response_buf.len() as _);
    response_buf.extend_from_slice(PONG_RSP);
    Ok(())
}
