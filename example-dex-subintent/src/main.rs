use std::str::FromStr;
use radix_common::address::AddressBech32Decoder;
use radix_common::network::NetworkDefinition;
use radix_common::types::ComponentAddress;
use radix_common_derive::dec;
use radix_transactions::manifest::decompile;
use radix_transactions::model::SubintentManifestV2;
use anthic_client::AnthicClient;
use anthic_model::{AnthicAddressInfo, AnthicConfig};
use anthic_subintents::*;

#[tokio::main]
async fn main() {
    let network = NetworkDefinition::from_str("stokenet").unwrap();
    let trade_api_url = "https://trade-api.staging.anthic.io";
    let anthic_api_key = "<YOUR ANTHIC-API-KEY>";
    let user_address = {
        let decoder = AddressBech32Decoder::new(&network);
        ComponentAddress::try_from_bech32(&decoder, "<A USERS ACCOUNT ADDRESS>").unwrap()
    };

    // A high level Anthic client which wraps calls to the Anthic API
    let client = AnthicClient::new(
        network.clone(),
        trade_api_url.to_string(),
        anthic_api_key.to_string()
    );

    // Anthic configuration
    let anthic_config = client.load_anthic_config().await.unwrap();

    // Address info
    let address_info = client.load_account_address_info(user_address).await.unwrap();

    // A user order to receive 0.001 Test-xwBTC in exchange for 95.85 Test-xUSDC
    let buy = TokenAmount {
        symbol: "Test-xwBTC".to_string(),
        amount: dec!("0.001"),
    };
    let sell = TokenAmount {
        symbol: "Test-xUSDC".to_string(),
        amount: dec!("95.85"),
    };

    // Create the manifest for the order
    let manifest = create_order_manifest(&anthic_config, user_address, &address_info, sell, buy).unwrap();

    println!("{}", decompile(&manifest, &network).unwrap());
}

fn create_order_manifest(
    anthic_config: &AnthicConfig,
    account_address: ComponentAddress,
    address_info: &AnthicAddressInfo,
    sell: TokenAmount,
    buy: TokenAmount
) -> Result<SubintentManifestV2, String> {
    let builder = AnthicSubintentManifestBuilder::new(anthic_config.clone());

    // There is a flat solver fee which includes transaction execution fee, a portion of which will be rebated
    // in the transaction
    let solver_fee_amount = anthic_config.settlement_fee_per_resource.get(&sell.symbol).unwrap().clone();
    let anthic_fee_percent = anthic_config.anthic_fee_per_level.get(address_info.level as usize).unwrap().taker_fee.clone();
    let anthic_fee_amount = sell.amount * anthic_fee_percent;

    let manifest = builder.add_anthic_limit_order(account_address, sell, buy, solver_fee_amount, anthic_fee_amount).build();
    Ok(manifest)
}