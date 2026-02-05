# Moonbeam Bridge Message Decoder

Decode `BridgeMessage` payloads from Moonbeam/Moonriver cross-chain bridge messages.

## Features

- Decodes SCALE-encoded XCM bridge messages
- Displays `AccountKey20`, `AccountId32`, and `SetTopic` as human-readable hex strings (e.g., `0x1c7fb1cbabcd...`)
- Supports nested XCM instructions (`SetAppendix`, `SetErrorHandler`)

## Usage

Pass the raw hex-encoded bridge message as a command-line argument:

```bash
cargo run -- <hex_data>
```

### Example

```bash
cargo run -- "0xb9100502090200511f05202509030b01009d1f0b0103010398891e5fd24ef33a488a47101f65d212ff6e650e00040001040a001300008a5d784563..."
```

You can include or omit the `0x` prefix.

## Message Format

The raw message has a 2-byte compact length prefix that is automatically skipped:

```
[0x31 0x02] [BridgeMessage payload...]
 ^^^^^^^^    ^^^^^^^^^^^^^^^^^^^^^^^
 length      actual data
 prefix
```

## Example Output

```
Destination: X2([GlobalConsensus(Polkadot), Parachain(2004)])

XCM Instructions:
  [0] UniversalOrigin(GlobalConsensus(Kusama))
  [1] DescendOrigin(X1([Parachain(2023)]))
  [2] DescendOrigin(X1([AccountKey20 { network: Some(Kusama), key: 0x98891e5fd24ef33a488a47101f65d212ff6e650e }]))
  [3] WithdrawAsset(Assets([Asset { id: AssetId(Location { parents: 0, interior: X1([PalletInstance(10)]) }), fun: Fungible(100000000000000000) }]))
  [4] BuyExecution { fees: Asset { ... }, weight_limit: Unlimited }
  [5] Transact { origin_kind: SovereignAccount, fallback_max_weight: None, call: "0x6d0001655e15..." }
  [6] SetAppendix(Xcm([RefundSurplus, DepositAsset { assets: Wild(AllCounted(1)), beneficiary: Location { parents: 0, interior: X1([AccountKey20 { network: None, key: 0x1c7fb1cbabcd242804342a3a5ebff63f0c701742 }]) } }]))
  [7] SetTopic(0x3cae78e536bbb2722be874d4c6a7b2fff82052a5cd4312bca7f1a7893f3ef5b8)
```

## Building

```bash
cargo build --release
```
