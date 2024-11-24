use std::collections::HashMap;
use radix_common::prelude::*;
use radix_engine_interface::prelude::*;
use anthic_model::*;
use anthic_trade_api_client::AnthicTradeApiClient;

/// A high level wrapper around the anthic api
pub struct AnthicClient {
    pub network: NetworkDefinition,
    pub decoder: AddressBech32Decoder,
    pub api_client: AnthicTradeApiClient,
}

impl AnthicClient {
    pub fn new(network: NetworkDefinition, url: String, api_key: String) -> Self {
        let decoder = AddressBech32Decoder::new(&network);
        Self {
            network,
            decoder,
            api_client: AnthicTradeApiClient::new(url, api_key),
        }
    }

    /// Loads various static configurations from the Anthic API
    pub async fn load_anthic_config(&self) -> Result<AnthicConfig, reqwest::Error> {
        let access_rule: AccessRule = {
            let anthic_info = self.api_client.info().await?;
            scrypto_decode(&hex::decode(anthic_info.verify_parent_access_rule_sbor_hex).unwrap()).unwrap()
        };

        let symbol_to_resource: HashMap<String, ResourceAddress> = {
            let tokens_response = self.api_client.tokens().await?;
            tokens_response.tokens.into_iter().map(|t| {
                let address = ResourceAddress::try_from_bech32(&self.decoder, &t.resource_address).unwrap();
                (t.symbol, address)
            }).collect()
        };


        let solver_fee_per_resource = {
            let fee_info = self.api_client.fee_info().await?;
            fee_info.solver_fee.into_iter().map(|info| {
                (info.symbol, Decimal::from_str(&info.amount).unwrap())
            }).collect()
        };

        Ok(AnthicConfig {
            verify_parent_access_rule: access_rule,
            solver_fee_per_resource,
            symbol_to_resource,
        })
    }

    /// Loads instamint configuration
    pub async fn load_instamint_config(&self) -> Result<InstamintConfig, reqwest::Error> {
        let instamint_info = self.api_client.instamint_info().await?;
        let customer_badge_resource = ResourceAddress::try_from_bech32(&self.decoder, &instamint_info.customer_badge_resource).unwrap();
        let instamint_component = ComponentAddress::try_from_bech32(&self.decoder, &instamint_info.instamint_component).unwrap();
        Ok(InstamintConfig {
            customer_badge_resource,
            instamint_component
        })
    }

    /// If authenticated, loads the associated Anthic account
    pub async fn load_anthic_account(&self) -> Result<AnthicAccount, reqwest::Error> {
        let instamint_account = {
            let instamint_accounts = self.api_client.instamint_accounts().await?;
            instamint_accounts.accounts.into_iter().next().unwrap()
        };

        let account = ComponentAddress::try_from_bech32(&self.decoder, &instamint_account.address).unwrap();
        let sbor_encoded_local_id = instamint_account.customer_badge_non_fungible_local_ids.into_iter().next().unwrap();

        let customer_badge_local_id: NonFungibleLocalId = scrypto_decode(&hex::decode(sbor_encoded_local_id).unwrap()).unwrap();
        Ok(AnthicAccount {
            address: account,
            instamint_customer_badge_local_id: Some(customer_badge_local_id),
        })
    }
}
