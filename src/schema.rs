use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;

use stable_dira::msg::{
    AdminAddressesResponse, CW20DiraContractAddressResponse, CollateralPriceResponse,
    CollateralResponse, CollateralTokenDenomResponse, ExecuteMsg, InstantiateMsg,
    LiquidationHealthResponse, MintableHealthResponse, MintedDiraResponse, QueryMsg,
    StablecoinHealthResponse,
};

use schemars::JsonSchema;
use serde::Serialize;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");

    // Clean and recreate schema directory
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // Export schemas for standard types
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);

    // Export individual query response schemas
    let response_types = vec![
        ("collateral_response", schema_for!(CollateralResponse)),
        ("minted_dira_response", schema_for!(MintedDiraResponse)),
        (
            "stablecoin_health_response",
            schema_for!(StablecoinHealthResponse),
        ),
        (
            "collateral_price_response",
            schema_for!(CollateralPriceResponse),
        ),
        (
            "liquidation_health_response",
            schema_for!(LiquidationHealthResponse),
        ),
        (
            "mintable_health_response",
            schema_for!(MintableHealthResponse),
        ),
        (
            "admin_addresses_response",
            schema_for!(AdminAddressesResponse),
        ),
        (
            "collateral_token_denom_response",
            schema_for!(CollateralTokenDenomResponse),
        ),
        (
            "c_w20_dira_contract_address_response",
            schema_for!(CW20DiraContractAddressResponse),
        ),
    ];

    // Export individual files and store their paths
    let mut individual_files: Vec<PathBuf> = Vec::new();
    for (name, schema) in response_types.iter() {
        let file_name = format!("{}.json", name);
        let file_path = out_dir.join(&file_name);
        export_schema(schema, &out_dir);
        individual_files.push(file_path);
    }

    // Combine all query responses into a single schema file
    #[derive(Serialize, JsonSchema)] // Add JsonSchema here
    struct QueryResponseMsg {
        collateral_response: CollateralResponse,
        minted_dira_response: MintedDiraResponse,
        stablecoin_health_response: StablecoinHealthResponse,
        collateral_price_response: CollateralPriceResponse,
        liquidation_health_response: LiquidationHealthResponse,
        mintable_health_response: MintableHealthResponse,
        admin_addresses_response: AdminAddressesResponse,
        collateral_token_denom_response: CollateralTokenDenomResponse,
        cw20_dira_contract_address_response: CW20DiraContractAddressResponse,
    }

    export_schema(&schema_for!(QueryResponseMsg), &out_dir);
    println!("Combined query responses schema created.");

    // Delete individual query response files
    for file in individual_files {
        if let Err(e) = remove_file(&file) {
            eprintln!("Failed to delete file {:?}: {}", file, e);
        } else {
            println!("Deleted individual file: {:?}", file);
        }
    }
}
