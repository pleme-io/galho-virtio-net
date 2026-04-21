# VirtioNet seiva protocol

## Messages (client → driver)

```rust
pub enum Message<'a> {
    Tx(&'a [u8]),
    RxReturn { buffer_id: u32 },
}
```

`Tx` transmits a raw ethernet frame (layer 2). Lifetime of the slice is scoped to the IPC call — the driver copies into its DMA ring before returning.

`RxReturn` is a flow-control acknowledgement. After the client processes a received frame, it returns the `buffer_id` so the driver can reuse the DMA buffer.

## Replies (driver → client)

```rust
pub enum Reply<'a> {
    Rx { buffer_id: u32, frame: &'a [u8] },
    BackPressure { dropped: u32 },
    LinkState(LinkState),
    Err(DriverError),
}
```

`Rx` hands an incoming frame to the client. The `frame` slice is a borrow into DMA memory; the client must either process it inline or copy before the next IPC call on this endpoint.

`BackPressure` is the explicit drop counter — never silent.

`LinkState` fires on up/down transitions and on MAC address assignment.

## Feature negotiation

At `init`:

1. Read `device_features` from virtio config space.
2. Advertise `driver_features = device_features & SUPPORTED`.
3. Confirm via `virtio_features_ok`.

`SUPPORTED` = {F_MAC, F_MRG_RXBUF, F_CSUM}. Phase 3 adds TSO/CTRL_VQ.

## MAC assignment

The driver requests `(net-mac :pool pleme-fleet)` — a typed cap backed by the fleet's MAC pool manager. The driver does NOT get to choose its MAC; the pool manager hands out deterministic MACs based on the node identity. This prevents rogue drivers from spoofing.

## IRQ handling

One dedicated IRQ thread. On IRQ:

1. Read interrupt status from MMIO.
2. Process completed RX descriptors → emit `Reply::Rx`.
3. Process completed TX descriptors → unblock writers.
4. `dev_irq_ack`.

No allocation.
