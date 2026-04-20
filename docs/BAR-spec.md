**Bitcoin App Registry (BAR) – BRC-App Protocol**  
**Comprehensive Technical Documentation v1**

**Date:** April 19, 2026  
**Canonical Registry Address:** `bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew`  
**Protocol Identifier:** `brc-app`  
**Status:** v1 (Ready for Launch & Community Adoption)

---

### Table of Contents
1. [Introduction & Motivation](#1-introduction-motivation)  
2. [Core Design Philosophy](#2-core-design-philosophy)  
3. [Background: Why Bitcoin L1 + Ordinals?](#3-background-why-bitcoin-l1-ordinals)  
4. [Protocol Overview](#4-protocol-overview)  
5. [JSON Schema](#5-json-schema)  
6. [Supported Operations](#6-supported-operations)  
7. [Validation Rules for Indexers](#7-validation-rules-for-indexers)  
8. [Genesis Inscription (Ready to Inscribe)](#8-genesis-inscription-ready-to-inscribe)  
9. [How to Register, Update, or Transfer an App](#9-how-to-register-update-or-transfer-an-app)  
10. [Practical Costs & Tooling](#10-practical-costs-tooling)  
11. [Integration Guide (BBOX, Zapstore, Frontends)](#11-integration-guide)  
12. [Long-Term Roadmap & BITVMX Migration Path](#12-long-term-roadmap-bitvmx-migration-path)  
13. [FAQ](#13-faq)  
14. [References & Resources](#14-references-resources)

---

### 1. Introduction & Motivation

The centralized app stores (Apple App Store and Google Play) control distribution to over **3 billion devices**. A single regulatory request — as seen with the **China ban of Bitchat** in February 2026 — is enough to remove an app from an entire country’s store with zero appeal process.

Bitcoin was built to be **permissionless, borderless, and censorship-resistant money**. The Bitcoin ecosystem now requires the same for **software distribution**.

**Bitcoin App Registry (BAR)** is a pure Bitcoin Layer-1, no-gatekeeper, on-chain registry for open-source applications. It allows any publisher to:

- Register their app permanently on Bitcoin.
- Update metadata (version, build hash, description, links, etc.).
- Transfer ownership to a new maintainer.
- Keep a complete, immutable audit trail of every change.

BAR turns Bitcoin into a **neutral, sovereign discovery layer** for open-source tools — wallets, Lightning apps, mesh messengers, privacy tools, DeFi protocols, and any other FOSS project.

---

### 2. Core Design Philosophy

BAR is built on four unbreakable principles:

1. **No Gatekeepers** — No company, DAO, multisig, or third party can block, censor, or delete an entry.
2. **Publisher Sovereignty** — Only the current owner (controlled by their Taproot private key) can modify or transfer the app.
3. **Immutable History** — Every change is append-only. The full chain of updates remains permanently visible on Bitcoin.
4. **Decentralized Verification** — Anyone can run an independent indexer. There is no central server or single source of truth.

---

### 3. Background: Why Bitcoin L1 + Ordinals?

Bitcoin Ordinals (launched January 2023 by Casey Rodarmor) introduced the ability to attach arbitrary data to individual satoshis via Taproot inscriptions. This created the most mature, battle-tested metadata standard on Bitcoin L1.

BAR leverages this infrastructure because:
- Inscriptions are **permanently on-chain** and verifiable by any Bitcoin node.
- No smart contracts or L2s are required.
- The protocol is already widely supported by wallets (Unisat, Xverse, Sparrow) and indexers.
- It is append-only by nature — perfect for an auditable registry.

(For full Ordinals history, see the separate timeline in the project repository.)

---

### 4. Protocol Overview

**Name:** Bitcoin App Registry (BAR)  
**Protocol ID:** `brc-app` (following the BRC naming convention used by BRC-20, BRC-721, etc.)  
**Underlying Mechanism:** Taproot inscriptions (Ordinals)  
**Canonical Anchor:** One fixed Taproot address that serves as the discovery point for the entire registry.

All inscriptions are linked to or discoverable via this address:

**`bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew`**

---

### 5. JSON Schema (v1)

Every inscription must be valid JSON matching this schema:

```json
{
  "p": "brc-app",                    // Fixed protocol identifier
  "op": "genesis" | "register" | "update" | "transfer",
  "app_id": "string",                // Permanent unique ID (e.g. "bitchat", "samourai-wallet")
  "owner": "bc1p...taproot-address", // Current publisher's Taproot address
  "name": "string",
  "repo": "string (URL)",
  "description": "string",
  "license": "string",
  "version": "string",
  "build_hash": "string (sha256: recommended)",
  "platform": ["android", "ios", "web", "linux", ...],
  "chain_layer": "none" | "BTC" | "LN" | "Stacks" | "Rootstock" | "Starknet" | "other",
  "previous": "string (previous inscription ID)" | null,
  "timestamp": number                // Unix timestamp (recommended)
}
```

**Key Notes:**
- `chain_layer` replaces any previous “Bitcoin-aligned” flag and supports any layer or “none” for general FOSS.
- `previous` creates a verifiable linked-list of states for each `app_id`.

---

### 6. Supported Operations

| Operation   | Description                                      | Required Fields                     | Who Can Do It          |
|-------------|--------------------------------------------------|-------------------------------------|------------------------|
| `genesis`   | One-time registry initialization                 | `p`, `op`, `name`, `description`   | Anyone (once only)     |
| `register`  | Create a new app entry                           | All core fields                     | Any publisher          |
| `update`    | Modify metadata (version, hash, etc.)            | `op`, `app_id`, `owner`, `previous`| Current owner only     |
| `transfer`  | Change ownership                                 | `op`, `app_id`, `owner`, `previous`| Current owner only     |

---

### 7. Validation Rules for Indexers

Any developer can build their own BAR indexer. The rules are deterministic and simple:

1. The inscription must be created from the Taproot address declared in the `owner` field.
2. For `update` and `transfer`, `previous` must correctly reference the last valid inscription for that `app_id`.
3. Only the **latest valid inscription** per `app_id` represents the current canonical state.
4. Full history remains immutable and publicly auditable via the chain of `previous` references.

---

### 8. Genesis Inscription (Ready to Inscribe)

**Copy and inscribe this exact JSON to the registry address:**

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

---

### 9. How to Register, Update, or Transfer an App

**Tools needed:** Unisat, Xverse, or Sparrow Wallet (with Ordinals support).

1. Create a new Taproot address (your publishing key).
2. Prepare the JSON for the desired operation.
3. Inscribe it (text/JSON content).
4. The inscription is automatically picked up by indexers that follow the `brc-app` protocol.

Example **Update** JSON (new version + Lightning support):

```json
{
  "p": "brc-app",
  "op": "update",
  "app_id": "bitchat",
  "owner": "bc1pYourCurrentAddress...",
  "name": "Bitchat",
  "version": "1.4.2",
  "build_hash": "sha256:newhash...",
  "chain_layer": "LN",
  "previous": "previous-inscription-id",
  "timestamp": 1745094000
}
```

---

### 10. Practical Costs & Tooling

- Typical inscription size: < 800 bytes.
- Current cost (April 2026, low congestion): **$3 – $15 USD** per operation.
- No platform fees, no approvals, no 30% cuts.

---

### 11. Integration Guide

- **BBOX (bbox.lol)**: Can query the registry address and display latest states.
- **Zapstore**: Pull via Ordinals indexers and add Nostr discovery.
- Custom frontends: Use public Ordinals APIs (Unisat, Hiro, ordinals.com) filtered by `p: "brc-app"` and the canonical address.

---

### 12. Long-Term Roadmap & BITVMX Migration Path

**v1 (now)**: Pure Ordinals, append-only.
**v2**: Recursive inscriptions for screenshots, verifiable builds.

---

### 13. FAQ

**Q: Can anyone register an app?**  
A: Yes — completely permissionless.

**Q: Can a government or company remove my app?**  
A: No. Once inscribed, the record lives forever on Bitcoin.

**Q: What if I lose my private key?**  
A: Ownership is lost (same as any Bitcoin key). Always back up your publishing key.

**Q: Is this only for Bitcoin apps?**  
A: No — `chain_layer` supports “none” for any open-source project.

---

### 14. References & Resources

- Registry Address: `bc1p0saw6z028y7h6eag3w6hx5an6mk5ta8qk7wx2d3gtqtrty243uvqvjzvew`
- Ordinals Documentation: https://docs.ordinals.com
