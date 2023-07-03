// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;

pub(crate) type TransactionId = u16;
pub(crate) type UnitId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub transaction_id: TransactionId,
    pub unit_id: UnitId,
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
            slave: from.hdr.unit_id,
            request: from.pdu.into(),
        }
    }
}
