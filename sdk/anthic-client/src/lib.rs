use std::collections::HashMap;
use radix_common::prelude::*;
use radix_engine_interface::prelude::*;
use anthic_model::*;
use anthic_trade_api_client::AnthicTradeApiClient;

/// A high level wrapper around the anthic api
pub struct AnthicClient {
    pub network: NetworkDefinition,
    pub encoder: AddressBech32Encoder,
    pub decoder: AddressBech32Decoder,
    /// Low level anthic api client
    pub trade_api_client: AnthicTradeApiClient,
}

impl AnthicClient {
    pub fn new(network: NetworkDefinition, url: String, api_key: String) -> Self {
        let decoder = AddressBech32Decoder::new(&network);
        let encoder = AddressBech32Encoder::new(&network);
        Self {
            network,
            decoder,
            encoder,
            trade_api_client: AnthicTradeApiClient::new(url, api_key),
        }
    }

    /// Loads various static configurations from the Anthic API
    pub async fn load_anthic_config(&self) -> Result<AnthicConfig, reqwest::Error> {
        let (verify_parent_access_rule, anthic_fee_per_level, settlement_fee_per_resource)= {
            let anthic_info = self.trade_api_client.info().await?;
            let verify_parent_access_rule: AccessRule = scrypto_decode(&hex::decode(anthic_info.verify_parent_access_rule_sbor_hex).unwrap()).unwrap();
            let anthic_taker_fee_per_level = anthic_info.per_level_anthic_fee.into_iter()
                .map(|level| {
                    AnthicLevelFee {
                        taker_fee: Decimal::from_str(&level.taker_fee).unwrap(),
                        maker_fee: Decimal::from_str(&level.maker_fee).unwrap(),
                    }
                }).collect();
            let solver_fee_resources = anthic_info.per_token_settlement_fee.into_iter().map(|info| {
                (info.symbol, Decimal::from_str(&info.transaction_execution_amount).unwrap() + Decimal::from_str(&info.solver_amount).unwrap())
            }).collect();
            (verify_parent_access_rule, anthic_taker_fee_per_level, solver_fee_resources)
        };

        let symbol_to_resource: HashMap<String, ResourceAddress> = {
            let tokens_response = self.trade_api_client.tokens().await?;
            tokens_response.tokens.into_iter().map(|t| {
                let address = ResourceAddress::try_from_bech32(&self.decoder, &t.resource_address).unwrap();
                (t.symbol, address)
            }).collect()
        };

        Ok(AnthicConfig {
            verify_parent_access_rule,
            settlement_fee_per_resource,
            anthic_fee_per_level,
            symbol_to_resource,
        })
    }

    /// Loads instamint-loan-repayment configuration
    pub async fn load_instamint_config(&self) -> Result<InstamintConfig, reqwest::Error> {
        let instamint_info = self.trade_api_client.instamint_info().await?;
        let customer_badge_resource = ResourceAddress::try_from_bech32(&self.decoder, &instamint_info.customer_badge_resource).unwrap();
        let instamint_component = ComponentAddress::try_from_bech32(&self.decoder, &instamint_info.instamint_component).unwrap();
        Ok(InstamintConfig {
            customer_badge_resource,
            instamint_component
        })
    }

    pub async fn load_account_address_info(&self, account_address: ComponentAddress) -> Result<AnthicAddressInfo, reqwest::Error> {
        let address_info = self.trade_api_client.account_address_info(self.encoder.encode(account_address.as_bytes()).unwrap()).await?;
        Ok(AnthicAddressInfo {
            level: address_info.level,
        })
    }

    /// If authenticated, loads the associated Anthic account
    pub async fn load_anthic_account(&self) -> Result<AnthicAccount, reqwest::Error> {
        let instamint_account = {
            let instamint_accounts = self.trade_api_client.instamint_account().await?;
            instamint_accounts.account
        };

        let account = ComponentAddress::try_from_bech32(&self.decoder, &instamint_account.address).unwrap();
        let sbor_encoded_local_id = instamint_account.customer_badge_non_fungible_local_ids.into_iter().next().unwrap();

        let customer_badge_local_id: NonFungibleLocalId = scrypto_decode(&hex::decode(sbor_encoded_local_id).unwrap()).unwrap();
        Ok(AnthicAccount {
            address: account,
            instamint_customer_badge_local_id: Some(customer_badge_local_id),
        })
    }

    pub async fn load_instamint_payback_addresses(&self) -> Result<InstamintRepaymentInfo, reqwest::Error> {
        let payback_addresses = {
            let instamint_payback_addresses = self.trade_api_client.instamint_payback_addresses().await?;
            instamint_payback_addresses.payback_addresses
        };

        let tokens = self.trade_api_client.instamint_tokens().await?;
        let info: HashMap<String, Vec<InstamintTokenPaybackAddress>> = tokens.tokens.into_iter()
            .filter(|token| token.chain.eq("Radix"))
            .map(|token| {
                let repayment_addresses: Vec<_> = token.repayment_tokens.iter().flat_map(|repayment_token| {
                    payback_addresses.iter()
                        .filter_map(|payback_address| {
                            if payback_address.chain.eq(&repayment_token.chain) {
                                Some(InstamintTokenPaybackAddress {
                                    chain: repayment_token.chain.clone(),
                                    symbol: repayment_token.symbol.clone(),
                                    token_identifier: repayment_token.clone().address.map(TokenIdentifierOnChain::Address).unwrap_or(TokenIdentifierOnChain::Native),
                                    address: payback_address.address.clone(),
                                })
                            } else {
                                None
                            }
                        })
                }).collect();

            (token.symbol, repayment_addresses)
        }).collect();

        Ok(InstamintRepaymentInfo {
            info
        })
    }

    pub async fn get_instamint_balance(&self) -> Result<HashMap<String, Decimal>, reqwest::Error> {
        let balances = {
            let instamint_balance = self.trade_api_client.instamint_balance().await?;
            instamint_balance.balances
        };

        let outstanding_loans = balances.into_iter().map(|loan| {
            (loan.symbol, Decimal::from_str(&loan.amount).unwrap())
        }).collect();

        Ok(outstanding_loans)
    }
}
