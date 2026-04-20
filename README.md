# Bitcoin App Registry (BAR) – BRC-APP Protocol

**A permissionless, no-gatekeeper, publisher-controlled on-chain registry for open-source applications on Bitcoin L1.**

Built as a direct response to centralized app store censorship — including the April 2026 China ban of **Bitchat**.

---

## Why BAR Exists

Apple and Google control distribution to over 3 billion devices. One regulatory email is enough to remove any app from an entire country.

Bitcoin was created to be **permissionless money**. BAR extends that same ethos to **software distribution**.

BAR lets any open-source developer permanently register their app on Bitcoin L1, update metadata, transfer ownership, and maintain a full immutable history — **with zero gatekeepers**.

No company. No DAO. No multisig. No approvals.

## Key Features

- Pure Bitcoin L1 (Taproot inscriptions via Ordinals)
- Publisher retains full sovereignty (update & transfer ownership)
- Append-only immutable history (full audit trail)
- No platform fees, no 30% cuts, no review process
- Supports **any** open-source app (Bitcoin-native or general FOSS)
- `chain_layer` field for filtering (BTC, LN, Stacks, Rootstock, Starknet, none, etc.)
- Designed to be easily indexed by BBOX, Zapstore, or custom explorers

## Canonical Registry Address

**`bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew`**

This is the single discovery anchor for the entire BAR registry.

## Protocol Specification (v1)

**Protocol ID:** `brc-app`

Full documentation: [docs/BAR-Spec.md](./docs/BAR-Spec.md)

### JSON Schema

```json
{
  "p": "brc-app",
  "op": "genesis" | "register" | "update" | "transfer",
  "app_id": "string",
  "owner": "bc1p...taproot-address",
  "name": "string",
  "repo": "string (URL)",
  "description": "string",
  "license": "string",
  "version": "string",
  "build_hash": "string (sha256:...)",
  "platform": ["android", "ios", "web", "linux", ...],
  "chain_layer": "none" | "BTC" | "LN" | "Stacks" | "Rootstock" | "Starknet" | "other",
  "previous": "string (previous inscription ID)" | null,
  "timestamp": number
}
```

## Genesis Inscription 

```json
{
  "p": "brc-app",
  "op": "genesis",
  "name": "Bitcoin App Registry (BAR)",
  "description": "Permissionless, no-gatekeeper on-chain registry of open-source applications on Bitcoin L1. Publishers retain full control to update metadata and transfer ownership. Append-only immutable history. Built as a censorship-resistant alternative after events like the Bitchat China ban.",
  "creator": "community-driven",
  "registry_address": "bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew",
  "timestamp": 1745092800
}
```
## Roadmap

- **v1 (Current)** – Pure Ordinals, append-only registry
- **v2** – Recursive inscriptions for screenshots + verifiable builds

## Contributing

This is a community-driven protocol. Contributions welcome:

- Improve the spec
- Build indexers / explorers
- Add sample inscriptions
- Integrate with BBOX or Zapstore

See [CONTRIBUTING.md](./CONTRIBUTING.md)

## License

MIT License – fully open source.

---

**BAR is not just another directory.**  
It is **sovereign infrastructure** for the open-source Bitcoin ecosystem.

Once inscribed, no government, no corporation, and no centralized store can ever erase the canonical record of your app.

**Time to inscribe the genesis and ship the first censorship-resistant app registry on Bitcoin.**

---

*Made by plebs for plebs*
