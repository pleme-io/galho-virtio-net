# galho-virtio-net — operator instructions

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical specification at [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md). The Compounding Directive (operational rules: solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in the org-level pleme-io/CLAUDE.md ★★★ section. Read both before non-trivial changes.


virtio-net driver. Layer 2 only. Packet-in / packet-out. No IP stack.

## Non-negotiables

- No allocation in packet fast path. RX/TX rings pre-allocated at init.
- MTU is fixed at init from `:caps-requested` configuration; changes require restart.
- Packet borrow lifetimes are scoped to a single IPC turn. Clients that need longer retention must copy.
- No cross-queue sharing. Each RX queue has one owning endpoint.

## Authoring surface

```lisp
(defdriver :name virtio-net
           :bus :pci
           :match {:vendor 0x1af4 :device 0x1000}
           :caps-requested [(mmio :device-bound)
                            (dma :size 1MB)
                            (irq :any)
                            (net-mac :pool pleme-fleet)]
           :protocol virtio-net
           :impl (rust-crate "galho-virtio-net"))
```

## Feature matrix

| virtio feature | phase 1 support |
|---|---|
| VIRTIO_NET_F_MAC | required |
| VIRTIO_NET_F_MRG_RXBUF | required |
| VIRTIO_NET_F_CSUM | optional (negotiated if host advertises) |
| VIRTIO_NET_F_HOST_TSO4/6 | deferred to Phase 3 |
| VIRTIO_NET_F_CTRL_VQ | Phase 3 (link state, MAC filter updates) |
| VIRTIO_NET_F_MQ | Phase 4 (multi-queue) |
