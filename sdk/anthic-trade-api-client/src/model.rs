use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct NetworkStatusResponse {
    pub cur_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct InfoResponse {
    pub verify_parent_access_rule_sbor_hex: String,
    pub per_token_settlement_fee: Vec<SettlementFeeItem>,
    pub per_level_anthic_fee: Vec<AnthicLevelFee>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SettlementFeeItem {
    pub symbol: String,
    pub solver_amount: String,
    pub transaction_execution_amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AnthicLevelFee {
    pub taker_fee: String,
    pub maker_fee: String,
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountAddressInfo {
    pub level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TokensResponse {
    pub tokens: Vec<TokenDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct TokenPair {
    /// Token symbol of the base resource
    pub base: String,
    /// Token symbol of the quote resource
    pub quote: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TokenPairsResponse {
    pub token_pairs: Vec<TokenPair>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountsResponse {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub balances: Vec<TokenAmount>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintInfo {
    pub instamint_component: String,
    pub customer_badge_resource: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintTokensResponse {
    pub tokens: Vec<InstamintTokenWithRepaymentInfo>,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstamintTokenWithRepaymentInfo {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub chain: String,
    pub repayment_tokens: Vec<InstamintToken>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstamintToken {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub chain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testnet: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintAccountResponse {
    pub account: InstamintAccount,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintAccount {
    pub customer_badge_non_fungible_local_ids: Vec<String>,
    pub address: String,
    pub payback_addresses: Vec<InstamintPaybackAddress>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintAllowance {
    pub allowance: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintBalance {
    pub balances: Vec<TokenAmount>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintPaybackAddresses {
    pub payback_addresses: Vec<InstamintPaybackAddress>,
}


#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InstamintPaybackAddress {
    pub chain: String,
    pub address: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenAmount {
    pub symbol: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct TokenDefinition {
    pub resource_address: String,
    pub symbol: String,
}
