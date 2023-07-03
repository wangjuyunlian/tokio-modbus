// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;

use crate::slave::SlaveId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub slave_id: SlaveId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestAdu {
    pub hdr: Header,
    pub pdu: RequestPdu,
    pub disconnect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseAdu {
    pub hdr: Header,
    pub pdu: ResponsePdu,
}

impl From<RequestAdu> for Request {
    fn from(from: RequestAdu) -> Self {
        from.pdu.into()
    }
}

#[cfg(feature = "server")]
impl From<RequestAdu> for SlaveRequest {
    fn from(from: RequestAdu) -> Self {
        Self {
            slave: from.hdr.slave_id,
            request: from.pdu.into(),
        }
    }
}
