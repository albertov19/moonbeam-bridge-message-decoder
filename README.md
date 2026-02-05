# Moonbeam Bridge Message Decoder

Decode `BridgeMessage` payloads from Moonbeam/Moonriver cross-chain bridge messages.

## Usage

1. Replace the hex payload in `src/main.rs`:

```rust
let raw = hex!(
    "YOUR_HEX_PAYLOAD_HERE"
);
```

2. Run:

```bash
cargo run
```

## Message Format

The raw message has a 2-byte compact length prefix that must be skipped:

```
[0x31 0x02] [BridgeMessage payload...]
 ^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^
 length      actual data (140 bytes)
 prefix
```

## Example Output

```
Destination: X2([GlobalConsensus(Polkadot), Parachain(2004)])

XCM Instructions:
  [0] UniversalOrigin(GlobalConsensus(Kusama))
  [1] DescendOrigin(X1([Parachain(2023)]))
  [2] WithdrawAsset(...)
  [3] ClearOrigin
  [4] BuyExecution { ... }
  [5] DepositAsset { ... }
  [6] SetTopic(...)
```
