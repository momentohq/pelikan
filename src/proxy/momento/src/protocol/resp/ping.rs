// Copyright 2022 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::protocol::resp::update_method_metrics;
use crate::Error;
use momento::SimpleCacheClient;
use protocol_resp::{Ping, PING, PING_EX};

const PONG_RSP: &[u8; 7] = b"+PONG\r\n";

pub async fn ping(
    client: &mut SimpleCacheClient,
    cache_name: &str,
    response_buf: &mut Vec<u8>,
    req: &Ping,
) -> Result<(), Error> {
    update_method_metrics(&PING, &PING_EX, async move {
        response_buf.extend_from_slice(PONG_RSP);
        Ok(())
    })
    .await
}
