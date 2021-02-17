#![cfg_attr(not(feature = "std"), no_std)]
/// this contract is ink! implementation of SubsCrpt. more information here: https://github.com/w3f/Open-Grants-Program/blob/master/applications/SubsCrypt.md


use ink_lang as ink;

use ink_storage::collections::HashMap;

#[ink::contract]
mod subscrypt {
    use ink_storage::{collections};
    use ink_storage::collections::HashMap;
    use ink_primitives::Key;
    use ink_env::{Error as Er, AccountId as Account, debug_println};
    use ink_env::hash::{Sha2x256};
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    use core::convert::TryInto;
    use ink_prelude::string::String;

    /// this struct represents a subscription record
    /// # fields:
    /// * provider : provider address of this record
    /// * plan : plen consts
    /// * plan_index : index of this plan in provider plans
    /// * subscription_time : this stores start time of subscription
    /// * meta_data_encrypted : user metadata with encryption
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, scale_info::TypeInfo)]
    pub struct SubscriptionRecord {
        provider: u128,
        plan: PlanConsts,
        plan_index: u128,
        subscription_time: u64,
        meta_data_encrypted: String,
        //encrypted Data with public key of provider
        refunded: bool,
    }

    /// this struct stores user plan records
    /// # fields:
    /// * subscription_records : history of subscription
    /// * pass_hash : hash of (token + pass_phrase) for authenticating user without wallet
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, scale_info::TypeInfo)]
    struct PlanRecord {
        subscription_records: Vec<SubscriptionRecord>,
        pass_hash: [u8; 32],
    }

    /// this struct stores data of plans
    /// # fields:
    /// * duration : duration of plan
    /// * active_session_limit :
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, scale_info::TypeInfo)]
    struct PlanConsts {
        duration: u64,
        active_session_limit: u128,
        price: u128,
        max_refund_percent_policy: u128,
        disabled: bool,
    }

    /// this struct represents a provider
    /// # fields:
    /// * plans : array of plans that this provider have
    /// * money_address : provider earned money will be sent to this address
    /// * payment_manager : struct for handling refund requests
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, scale_info::TypeInfo)]
    struct Provider {
        plans: Vec<PlanConsts>,
        money_address: u128,
        payment_manager: LinkedList,
    }

    /// this struct represents a user
    /// # fields:
    /// * list_of_providers : list of providers
    /// * joined_date : when this user joined the platform
    /// * subs_crypt_pass_hash : pass hash for retrieve data
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug, scale_info::TypeInfo)]
    struct User {
        list_of_providers: Vec<u128>,
        joined_date: u64,
        subs_crypt_pass_hash: [u8; 32],
    }

    /// struct for handling payments of refund
    /// * head : head of linked list
    /// * back : back of linked list
    /// * length : length of linked list
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, scale_info::TypeInfo)]
    struct LinkedList {
        head: u64,
        back: u64,
        length: u128,
    }

    /// struct that represents a payment admission
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, scale_info::TypeInfo)]
    struct Object {
        number: u128,
        next_day: u64,
    }

    /// main struct of contract
    /// # fields:
    /// * index_counter : counter for index_to_address hashmap
    /// * start_time : start time of contract
    /// * provider_register_fee : each provider should pay the fee to use contract
    /// * index_to_address and address_to_index : for indexing addresses
    /// * providers : the hashmap that stores providers data
    /// * users : the hashmap that stores users data
    /// * objects : the hashmap that stores payment admissions data
    /// * records : the hashmap that stores user's subscription records data
    /// * plan_index_to_record_index : the hashmap that stores user's last plan index for each plan index
    #[ink(storage)]
    pub struct Subscrypt {
        index_counter: u128,
        start_time: u64,
        provider_register_fee: u128,
        index_to_address: HashMap<u128, Account>,
        // index -> AccountId
        address_to_index: HashMap<Account, u128>,
        // AccountId -> index
        providers: HashMap<Account, Provider>,
        // (provider AccountId) -> provider data
        users: HashMap<Account, User>,
        // (user AccountId) -> user data
        objects: HashMap<(Account, u64), Object>,
        // (provider AccountId , day_id) -> payment admission
        records: HashMap<(Account, Account), PlanRecord>,
        // (user AccountId, provider AccountId) -> PlanRecord struct
        plan_index_to_record_index: HashMap<(Account, Account, u128), u128>,// (user AccountId, provider AccountId, plan_index) -> index
    }
}