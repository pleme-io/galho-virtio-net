# galho-virtio-net

> virtio-net driver — packet send/receive for brasa, spoken as a typed seiva protocol.

A userspace driver for the virtio-net transport (VIRTIO 1.2, §5.1). Exposes a typed packet-passing endpoint; does *not* include an IP stack — that lives in a separate service (planned: `rede-core`, smoltcp-wrapped).

**Status:** Phase 0 — Design.
**License:** MIT.

## What it does

- Enumerates virtio-net PCI devices.
- Negotiates features (MAC, MRG_RXBUF, CSUM, HOST_TSO4/6).
- Manages RX/TX vrings with mergeable RX buffers.
- Forwards packets as `Message::Tx(frame)` / `Reply::Rx(frame)` over a `seiva::Endpoint<VirtioNet>`.

## What it does *not* do

- No IP, no TCP, no UDP. This is strictly a layer-2 driver.
- No ARP, no DHCP. A separate network service handles those.
- No bonding, no VLAN tagging. Those belong in a layer above.

Keeping the driver surface minimal is a brasa principle: one thing, well-typed.

## Dependencies

Same as all galho-* drivers — `galho`, `casca`, `seiva`, `raiz`, `folha` from [brasa](https://github.com/pleme-io/brasa).

## See also

- [`docs/protocol.md`](./docs/protocol.md)
- [virtio-v1.2 §5.1](https://docs.oasis-open.org/virtio/virtio/v1.2/os-virtio-v1.2.html)
