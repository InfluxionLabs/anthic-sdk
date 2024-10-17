use radix_common::prelude::*;
use radix_engine_interface::prelude::*;
use radix_transactions::prelude::*;

fn main() {
    let limit_order = {
        let account_address = "account_sim12y62wm8w0kam5xt5rq9uqf49kja2v4gktgxuxt49z9xteetjuguqyy";
        let buy_resource = "resource_sim1t46kz5luvdz6rugagxx7ksst22xud8nj3mmldh2d92tvxsgd9pz9p0";
        let sell_resource = "resource_sim1tkktsemn6sx6fkfxt8jgykw4c9nh9q59pdlm30q5jqz27fn9awrqy7";

        let decoder = AddressBech32Decoder::for_simulator();
        let account = ComponentAddress::try_from_bech32(&decoder, account_address).unwrap();
        let buy_resource = ResourceAddress::try_from_bech32(&decoder, buy_resource).unwrap();
        let sell_resource = ResourceAddress::try_from_bech32(&decoder, sell_resource).unwrap();

        LimitOrderDefinition {
            account,
            buy: ResourceAmount {
                resource: buy_resource,
                amount: dec!("1.5")
            },
            sell: ResourceAmount {
                resource: sell_resource,
                amount: dec!(4000)
            },
            use_instamint: false,
        }
    };

    let subintent = compose_limit_order_subintent(limit_order, 999);
    let signature = {
        let prepared_subintent = subintent.prepare(PreparationSettings::latest_ref()).unwrap();
        let hash = prepared_subintent.subintent_hash();
        let private_key = Secp256k1PrivateKey::from_u64(1).unwrap();
        private_key.sign(&hash)
    };

    let signed_partial_transaction_hex = {
        let signed_partial_transaction = create_signed_partial_transaction(subintent, signature);
        hex::encode(manifest_encode(&signed_partial_transaction).unwrap())
    };

    println!("signed_partial_transaction: {}", signed_partial_transaction_hex);
}

fn create_signed_partial_transaction(subintent: SubintentV2, signature: Secp256k1Signature) -> SignedPartialTransactionV2 {
    SignedPartialTransactionV2 {
        partial_transaction: PartialTransactionV2 {
            root_subintent: subintent,
            non_root_subintents: NonRootSubintentsV2(Default::default()),
        },
        root_subintent_signatures: IntentSignaturesV2 {
            signatures: vec![IntentSignatureV1(
                SignatureWithPublicKeyV1::Secp256k1 {
                    signature
                }
            )],
        },
        non_root_subintent_signatures: NonRootSubintentSignaturesV2 {
            by_subintent: Default::default(),
        },
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ResourceAmount {
    pub resource: ResourceAddress,
    pub amount: Decimal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderDefinition {
    pub account: ComponentAddress,
    pub buy: ResourceAmount,
    pub sell: ResourceAmount,
    pub use_instamint: bool,
}

pub fn compose_limit_order_subintent(order: LimitOrderDefinition, id: u64) -> SubintentV2 {
    let manifest = compose_limit_order_manifest(order);
    create_subintent_from_manifest(manifest, id)
}

pub fn compose_limit_order_manifest(order: LimitOrderDefinition) -> SubintentManifestV2 {
    let builder = if order.use_instamint {
        ManifestBuilder::new_subintent_v2()
            .mint_fungible(order.sell.resource, order.sell.amount)
            .deposit_entire_worktop(order.account)
    } else {
        ManifestBuilder::new_subintent_v2()
    };

    builder
        .withdraw_from_account(order.account, order.sell.resource, order.sell.amount)
        .take_from_worktop(order.sell.resource, order.sell.amount, "sell")
        .assert_bucket_contents(
            "sell",
            ManifestResourceConstraint::ExactAmount(order.sell.amount),
        )
        .assert_next_call_returns_only(ManifestResourceConstraints::new().with(
            order.buy.resource,
            ManifestResourceConstraint::AtLeastAmount(order.buy.amount),
        ))
        .with_bucket("sell", |builder, bucket| builder.yield_to_parent((bucket,)))
        .deposit_entire_worktop(order.account)
        .yield_to_parent(())
        .build()
}

pub fn create_subintent_from_manifest(manifest: SubintentManifestV2, id: u64) -> SubintentV2 {
    let (instructions, blobs, children) = manifest.for_intent();
    SubintentV2 {
        intent_core: IntentCoreV2 {
            header: IntentHeaderV2 {
                network_id: NetworkDefinition::simulator().id,
                start_epoch_inclusive: Epoch::zero(),
                end_epoch_exclusive: Epoch::of(100),
                min_proposer_timestamp_inclusive: None,
                max_proposer_timestamp_exclusive: None,
                intent_discriminator: id,
            },
            blobs,
            message: Default::default(),
            children,
            instructions,
        },
    }
}
