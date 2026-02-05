# Moonbeam Bridge Message Decoder

Decode `BridgeMessage` payloads from Moonbeam/Moonriver cross-chain bridge messages.

## Usage

### Run

```bash
cargo run -- 0x31020502090200511f051c2509030b01009d1f00080001040a000f0000434fd7946a0002046e0300931715fee2d06333043d11f658c8ce934ac61d0c00828d5b000a130001040a000f0000434fd7946a000d010208000103001c7fb1cbabcd242804342a3a5ebff63f0c7017422cfbce4f422ce0d901e2131b36c1480582eab6db134e6caf41e10ae1cbd6c3f509
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
