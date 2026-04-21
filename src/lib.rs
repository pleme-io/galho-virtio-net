//! # galho-virtio-net
//!
//! virtio-net driver. Layer-2 packet I/O; no IP stack here.
//!
//! See [`docs/protocol.md`](../docs/protocol.md) for the seiva protocol.

#![cfg_attr(not(feature = "std"), no_std)]

use galho::{Driver, DriverError, DeviceHandle};

pub const VIRTIO_VENDOR: u16 = 0x1af4;
pub const VIRTIO_NET_DEVICE: u16 = 0x1000;

/// Layer-2 frame messages passed over the seiva endpoint.
///
/// Phase 1: zero-copy frame passing via DMA-backed `MemCap` buffers; the
/// `&[u8]` here is a live borrow into mapped DMA memory.
pub enum Message<'a> {
    /// Transmit this ethernet frame.
    Tx(&'a [u8]),
    /// Return an RX buffer to the driver (flow control).
    RxReturn { buffer_id: u32 },
}

pub enum Reply<'a> {
    /// Incoming ethernet frame; buffer is owned by driver, valid until
    /// client returns `Message::RxReturn { buffer_id }`.
    Rx { buffer_id: u32, frame: &'a [u8] },
    /// TX queue saturated; counts dropped frames since last ack.
    BackPressure { dropped: u32 },
    /// Link state changed.
    LinkState(LinkState),
    Err(DriverError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkState {
    Up { speed_mbps: u32, mac: MacAddr },
    Down,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MacAddr(pub [u8; 6]);

pub struct VirtioNet {
    // Phase 1: ring state, DMA buffers, IRQ waiter, MAC, negotiated features.
}

impl Driver for VirtioNet {
    fn init(&mut self) -> Result<(), DriverError> { Err(DriverError::Unsupported) }
    fn attach(&mut self, _: DeviceHandle) -> Result<(), DriverError> { Err(DriverError::Unsupported) }
    fn detach(&mut self) -> Result<(), DriverError> { Err(DriverError::Unsupported) }
}
