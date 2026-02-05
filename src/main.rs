use hex_literal::hex;
use parity_scale_codec::Decode;
use staging_xcm::{VersionedInteriorLocation, VersionedXcm};
use staging_xcm_builder::BridgeMessage;

fn main() {
    // Raw message data (includes 2-byte compact length prefix)
    let raw = hex!(
        "31020502090200511f051c2509030b01009d1f00080001040a000f0000434fd7946a0002046e03"
        "00931715fee2d06333043d11f658c8ce934ac61d0c00828d5b000a130001040a000f0000434fd7"
        "946a000d010208000103001c7fb1cbabcd242804342a3a5ebff63f0c7017422cfbce4f422ce0d9"
        "01e2131b36c1480582eab6db134e6caf41e10ae1cbd6c3f509"
    );

    // Skip 2-byte compact length prefix
    let payload = &raw[2..];

    let msg = BridgeMessage::decode(&mut &payload[..]).expect("decode failed");

    // Print destination
    if let VersionedInteriorLocation::V5(dest) = &msg.universal_dest {
        println!("Destination: {dest:?}");
    }

    // Print XCM instructions
    if let VersionedXcm::V5(xcm) = &msg.message {
        println!("\nXCM Instructions:");
        for (i, instr) in xcm.0.iter().enumerate() {
            println!("  [{i}] {instr:?}");
        }
    }
}
