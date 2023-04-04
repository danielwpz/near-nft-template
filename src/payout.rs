use std::collections::HashMap;

use near_sdk::{
    assert_one_yocto,
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{types::apply_basis_point, *};

/// A mapping of NEAR accounts to the amount each should be paid out, in
/// the event of a token-sale. The payout mapping MUST be shorter than the
/// maximum length specified by the financial contract obtaining this
/// payout data. Any mapping of length 10 or less MUST be accepted by
/// financial contracts, so 10 is a safe upper limit.

/// This currently deviates from the standard but is in the process of updating to use this type
#[derive(Default, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    payout: HashMap<AccountId, U128>,
}

pub trait Payouts {
    /// Given a `token_id` and NEAR-denominated balance, return the `Payout`.
    /// struct for the given token. Panic if the length of the payout exceeds
    /// `max_len_payout.`
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: Option<u32>) -> Payout;
    /// Given a `token_id` and NEAR-denominated balance, transfer the token
    /// and return the `Payout` struct for the given token. Panic if the
    /// length of the payout exceeds `max_len_payout.`
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        approval_id: Option<u64>,
        memo: Option<String>,
        balance: U128,
        max_len_payout: Option<u32>,
    ) -> Payout;
}

#[near_bindgen]
impl Payouts for Contract {
    fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: Option<u32>) -> Payout {
        let max_len_payout = max_len_payout.unwrap_or(10);
        require!(max_len_payout >= 1, "Bad max len payout");

        let token_owner_id = self
            .tokens
            .owner_by_id
            .get(&token_id)
            .expect("Token not exist");
        let mut payouts: HashMap<AccountId, U128> = HashMap::new();

        let creator_payout = if let Some(creator_id) = &self.creator_id {
            let creator_royalty_bp = self.creator_royalty_bp.unwrap();
            let payout = apply_basis_point(balance.0, creator_royalty_bp);
            payouts.insert(creator_id.clone(), payout.into());
            payout
        } else {
            // all payment goes to owner
            0 as u128
        };

        let owner_payout = balance.0 - creator_payout;
        payouts.insert(token_owner_id, owner_payout.into());

        Payout { payout: payouts }
    }

    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        approval_id: Option<u64>,
        memo: Option<String>,
        balance: U128,
        max_len_payout: Option<u32>,
    ) -> Payout {
        assert_one_yocto();
        let payout = self.nft_payout(token_id.clone(), balance, max_len_payout);
        self.nft_transfer(receiver_id, token_id, approval_id, memo);
        payout
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, ONE_NEAR};

    use crate::tests::sample_token_metadata;
    use crate::types::FULL_BASIS_POINT;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5990000000000000000000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_contract_meta() -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "foo".to_string(),
            symbol: "FOO".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_no_creator_royalty() {
        let owner = accounts(0);
        let alice = accounts(2);

        let mut context = get_context(owner.clone());
        testing_env!(context.build());
        let mut contract = Contract::new(owner.clone().into(), sample_contract_meta(), None, None);

        // mint one NFT
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(owner.clone())
            .build());

        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), alice.clone(), sample_token_metadata());

        let balance = ONE_NEAR;
        let payout = contract.nft_payout(token_id, balance.into(), Some(10));

        // alice should get all payment
        assert_eq!(payout.payout.len(), 1);
        assert_eq!(payout.payout.get(&alice).unwrap().0, balance);
    }

    #[test]
    fn test_creator_royalty() {
        let owner = accounts(0);
        let creator = accounts(1);
        let alice = accounts(2);

        let mut context = get_context(owner.clone());
        testing_env!(context.build());
        let mut contract = Contract::new(
            owner.clone().into(),
            sample_contract_meta(),
            Some(creator.clone()),
            Some(FULL_BASIS_POINT / 10),
        );

        // mint one NFT
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(owner.clone())
            .build());

        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), alice.clone(), sample_token_metadata());

        let balance = ONE_NEAR;
        let payout = contract.nft_payout(token_id, balance.into(), Some(10));

        // alice should get 90% and creator get 10%
        assert_eq!(payout.payout.len(), 2);
        assert_eq!(payout.payout.get(&alice).unwrap().0, balance * 9 / 10);
        assert_eq!(payout.payout.get(&creator).unwrap().0, balance / 10);
    }
}
