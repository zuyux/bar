# BAR Inscriber

This Rust CLI prepares BAR/BRC-App JSON payloads and sends them through `ord`.
It defaults to `--dry-run`; pass `--execute` only when your Bitcoin Core and
`ord` wallet are funded, synced, and ready to broadcast.

## Genesis

```bash
cargo run -- genesis --fee-rate 10
```

Broadcast the canonical genesis inscription to the BAR registry address:

```bash
cargo run -- genesis --fee-rate 10 --execute
```

## First BBOX App Registration

```bash
cargo run -- bbox \
  --fee-rate 10 \
  --owner bc1pYourPublisherTaprootAddress \
  --repo https://github.com/zuyux/bbox \
  --build-hash sha256:0000000000000000000000000000000000000000000000000000000000000000
```

Broadcast it:

```bash
cargo run -- bbox \
  --fee-rate 10 \
  --owner bc1pYourPublisherTaprootAddress \
  --repo https://github.com/zuyux/bbox \
  --build-hash sha256:0000000000000000000000000000000000000000000000000000000000000000 \
  --execute
```

Useful flags:

- `--network signet`, `--network testnet`, or `--network regtest`
- `--postage 330sats`
- `--destination <taproot-address>`
- `--print-only`
- repeat `--platform` for multiple platforms

The generated JSON files are written to `inscriptions/generated/`.
