use std::str::FromStr;
use radix_common::network::NetworkDefinition;
use anthic_client::AnthicClient;
use anthic_model::TokenIdentifierOnChain;

#[tokio::main]
async fn main() {
    let network = NetworkDefinition::from_str("stokenet").unwrap();
    let trade_api_url = "https://trade-api.staging.anthic.io";
    let anthic_api_key = "<YOUR ANTHIC-API-KEY>";
    let symbol = "xUSDT";
    let payback_chain = "Radix";

    // A high level Anthic client which wraps calls to the Anthic API
    let client = AnthicClient::new(
        network.clone(),
        trade_api_url.to_string(),
        anthic_api_key.to_string()
    );

    // Get repayment info for your account
    let repayment_info = client.load_instamint_payback_addresses().await.map_err(|e| println!("{:#?}", e)).unwrap();

    // Get all outstanding instamint loans
    let outstanding_loans = client.get_instamint_balance().await.unwrap();

    // Get the outstanding loan for xUSDT specifically
    let outstanding_xusdt_loan = outstanding_loans.get(symbol).cloned().unwrap_or_default();

    // Get the repayment address info for paying back xUSDT
    let repayment_address = repayment_info.get_repayment_address(symbol, payback_chain).unwrap();

    println!("Loan: {} {}", outstanding_xusdt_loan, symbol);
    println!("Payback Chain: {}", repayment_address.chain);
    println!("Payback Address: {}", repayment_address.address);
    println!("Payback Token: {} {}", repayment_address.symbol, match repayment_address.token_identifier {
        TokenIdentifierOnChain::Address(address) => address,
        TokenIdentifierOnChain::Native => "Native Token".to_string(),
    });
}