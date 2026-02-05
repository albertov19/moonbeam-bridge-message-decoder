use std::env;

use parity_scale_codec::Decode;
use staging_xcm::v5::prelude::*;
use staging_xcm::{VersionedInteriorLocation, VersionedXcm};
use staging_xcm_builder::BridgeMessage;

/// Formats a byte array as a hex string with 0x prefix
fn bytes_to_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

/// Formats a Junction with human-readable hex strings
fn format_junction(junction: &Junction) -> String {
    match junction {
        Junction::AccountKey20 { network, key } => {
            let network_str = match network {
                Some(n) => format!("Some({n:?})"),
                None => "None".to_string(),
            };
            format!("AccountKey20 {{ network: {network_str}, key: {} }}", bytes_to_hex(key))
        }
        Junction::AccountId32 { network, id } => {
            let network_str = match network {
                Some(n) => format!("Some({n:?})"),
                None => "None".to_string(),
            };
            format!("AccountId32 {{ network: {network_str}, id: {} }}", bytes_to_hex(id))
        }
        Junction::GeneralKey { length, data } => {
            format!("GeneralKey {{ length: {length}, data: {} }}", bytes_to_hex(&data[..*length as usize]))
        }
        other => format!("{other:?}"),
    }
}

/// Helper macro to reduce duplication in format_junctions
macro_rules! format_junction_array {
    ($arr:expr, $name:expr) => {{
        let items: Vec<String> = $arr.iter().map(format_junction).collect();
        format!("{}([{}])", $name, items.join(", "))
    }};
}

/// Formats Junctions with human-readable hex strings
fn format_junctions(junctions: &Junctions) -> String {
    match junctions {
        Junctions::Here => "Here".to_string(),
        Junctions::X1(arr) => format_junction_array!(arr, "X1"),
        Junctions::X2(arr) => format_junction_array!(arr, "X2"),
        Junctions::X3(arr) => format_junction_array!(arr, "X3"),
        Junctions::X4(arr) => format_junction_array!(arr, "X4"),
        Junctions::X5(arr) => format_junction_array!(arr, "X5"),
        Junctions::X6(arr) => format_junction_array!(arr, "X6"),
        Junctions::X7(arr) => format_junction_array!(arr, "X7"),
        Junctions::X8(arr) => format_junction_array!(arr, "X8"),
    }
}

/// Formats a Location with human-readable hex strings
fn format_location(location: &Location) -> String {
    format!(
        "Location {{ parents: {}, interior: {} }}",
        location.parents,
        format_junctions(&location.interior)
    )
}

/// Formats an Asset with human-readable hex strings
fn format_asset(asset: &Asset) -> String {
    format!(
        "Asset {{ id: AssetId({}), fun: {:?} }}",
        format_location(&asset.id.0),
        asset.fun
    )
}

/// Formats Assets with human-readable hex strings
fn format_assets(assets: &Assets) -> String {
    let items: Vec<String> = assets.inner().iter().map(format_asset).collect();
    format!("Assets([{}])", items.join(", "))
}

/// Formats an XCM instruction with human-readable hex strings
fn format_instruction(instr: &Instruction<()>) -> String {
    match instr {
        Instruction::WithdrawAsset(assets) => {
            format!("WithdrawAsset({})", format_assets(assets))
        }
        Instruction::DepositAsset { assets, beneficiary } => {
            format!(
                "DepositAsset {{ assets: {assets:?}, beneficiary: {} }}",
                format_location(beneficiary)
            )
        }
        Instruction::TransferAsset { assets, beneficiary } => {
            format!(
                "TransferAsset {{ assets: {}, beneficiary: {} }}",
                format_assets(assets),
                format_location(beneficiary)
            )
        }
        Instruction::BuyExecution { fees, weight_limit } => {
            format!(
                "BuyExecution {{ fees: {}, weight_limit: {weight_limit:?} }}",
                format_asset(fees)
            )
        }
        Instruction::SetTopic(topic) => {
            format!("SetTopic({})", bytes_to_hex(topic))
        }
        Instruction::DescendOrigin(junctions) => {
            format!("DescendOrigin({})", format_junctions(junctions))
        }
        Instruction::UniversalOrigin(junction) => {
            format!("UniversalOrigin({})", format_junction(junction))
        }
        Instruction::ReserveAssetDeposited(assets) => {
            format!("ReserveAssetDeposited({})", format_assets(assets))
        }
        Instruction::ReceiveTeleportedAsset(assets) => {
            format!("ReceiveTeleportedAsset({})", format_assets(assets))
        }
        Instruction::InitiateReserveWithdraw { assets, reserve, xcm: _ } => {
            format!(
                "InitiateReserveWithdraw {{ assets: {assets:?}, reserve: {}, xcm: ... }}",
                format_location(reserve)
            )
        }
        Instruction::InitiateTeleport { assets, dest, xcm: _ } => {
            format!(
                "InitiateTeleport {{ assets: {assets:?}, dest: {}, xcm: ... }}",
                format_location(dest)
            )
        }
        Instruction::ReportHolding { response_info, assets } => {
            format!(
                "ReportHolding {{ response_info: {{ destination: {}, query_id: {}, max_weight: {:?} }}, assets: {assets:?} }}",
                format_location(&response_info.destination),
                response_info.query_id,
                response_info.max_weight
            )
        }
        Instruction::QueryResponse { query_id, response, max_weight, querier } => {
            let querier_str = match querier {
                Some(loc) => format!("Some({})", format_location(loc)),
                None => "None".to_string(),
            };
            format!(
                "QueryResponse {{ query_id: {query_id}, response: {response:?}, max_weight: {max_weight:?}, querier: {querier_str} }}"
            )
        }
        // Default fallback for other instructions
        other => format!("{other:?}"),
    }
}

fn main() {
    // Get hex input from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <hex_data>", args[0]);
        eprintln!("Example: {} 0xb910050209...", args[0]);
        std::process::exit(1);
    }

    // Parse the hex string (remove 0x prefix if present)
    let hex_str = args[1].trim_start_matches("0x").trim_start_matches("0X");
    let raw = hex::decode(hex_str).expect("Invalid hex string");

    // Skip 2-byte compact length prefix
    let payload = &raw[2..];

    let msg = BridgeMessage::decode(&mut &payload[..]).expect("decode failed");

    // Print destination
    if let VersionedInteriorLocation::V5(dest) = &msg.universal_dest {
        println!("Destination: {}", format_junctions(dest));
    }

    // Print XCM instructions
    if let VersionedXcm::V5(xcm) = &msg.message {
        println!("\nXCM Instructions:");
        for (i, instr) in xcm.0.iter().enumerate() {
            println!("  [{i}] {}", format_instruction(instr));
        }
    }
}