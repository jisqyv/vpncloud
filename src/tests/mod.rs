// VpnCloud - Peer-to-Peer VPN
// Copyright (C) 2015-2021  Dennis Schwerdel
// This software is licensed under GPL-3 or newer (see LICENSE.md)

mod common;
mod nat;
mod payload;
mod peers;

#[test]
fn test_time_format() {
    assert!(time::OffsetDateTime::try_now_local().is_ok());
}
