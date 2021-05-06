// Copyright 2020-2021 OxyDev.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

#[ink_lang::contract]
pub mod subscrypt {
    use core::convert::TryInto;
    use ink_env::hash::Sha2x256;
    use ink_env::Error;
    use ink_prelude::string::String;
    use ink_prelude::vec;
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    /// This struct represents a subscription record
    /// # fields:
    /// * provider
    /// * plan
    /// * plan_index
    /// * subscription_time : this stores start time of each subscription (used in linkedList)
    /// * characteristics_values_encrypted : this is the features that user has chosen for her subscription
    /// * refunded
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SubscriptionRecord {
        pub provider: AccountId,
        pub plan: PlanConsts,
        pub plan_index: u128,
        subscription_time: u64,
        characteristics_values_encrypted: Vec<String>,
        //encrypted Data with public key of provider
        pub refunded: bool,
    }

    /// This struct stores user plan records
    /// # fields:
    /// * subscription_records
    /// * pass_hash : hash of (token + pass_phrase) for authenticating user without wallet
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PlanRecord {
        pub subscription_records: Vec<SubscriptionRecord>,
        pass_hash: [u8; 32],
    }

    /// This struct stores configs of plan which is set by provider
    /// # Note
    /// `max_refund_permille_policy` is out of 1000
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, Clone, Copy)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, std::cmp::PartialEq))]
    pub struct PlanConsts {
        pub duration: u64,
        pub(crate) price: u128,
        pub(crate) max_refund_permille_policy: u128,
        pub disabled: bool,
    }

    /// This struct represents a provider
    /// # fields:
    /// * plans
    /// * money_address : provider earned money will be sent to this address
    /// * payment_manager : struct for handling refund requests
    /// * subscrypt_pass_hash : password of provider to login into SubsCrypt Dashboard
    /// * plans_characteristics : array of key arrays of features of that plan
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Provider {
        pub plans: Vec<PlanConsts>,
        pub(crate) plans_characteristics: Vec<Vec<String>>,
        pub(crate) money_address: AccountId,
        payment_manager: LinkedList,
        pub subscrypt_pass_hash: [u8; 32],
    }

    /// This struct represents a user
    /// # fields:
    /// * list_of_providers : list of providers that the user subscribed to
    /// * subscrypt_pass_hash : pass hash for retrieve data in subscrypt user dashboard
    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct User {
        pub list_of_providers: Vec<AccountId>,
        pub subscrypt_pass_hash: [u8; 32],
    }

    /// Struct for handling payments of refund
    /// # Description
    ///
    /// This LinkedList is used for keeping tracking of each subscription that will end in some
    /// specific date in future. We order these subscriptions by their date of expiration, so we
    /// will be able to easily calculate and handle refund - withdraw methods with a minimum
    /// transaction fee. Each entity of the linked-list is `PaymentAdmission` struct.
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct LinkedList {
        pub head: u64,
        pub back: u64,
        pub length: u128,
    }

    /// Struct that represents amount of money that can be withdraw after its due date passed.
    #[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct DailyLockedAmount {
        amount: u128,
        next_day: u64,
    }

    pub struct ProcessReturningData {
        withdrawing_amount: u128,
        current_linked_list_head: u64,
        reduced_length: u128,
    }

    /// Main struct of contract
    /// # fields:
    /// * `start_time` : start time of the contract which is used in `LinkedList`
    /// * `provider_register_fee`
    /// * `providers` : the hashmap that stores providers data
    /// * `users` : the hashmap that stores users data
    /// * `daily_locked_amounts` : the hashmap that stores `DailyLockedAmount` data of each day in order
    /// * `records` : the hashmap that stores user's subscription records data
    /// * `plan_index_to_record_index` : the hashmap that stores user's last `SubscriptionRecord` index
    /// * `PlanRecord.subscription_records` for each (user, provider, plan_index)
    /// * `username_to_address` mapping of Usernames to Addresses
    /// * `address_to_username` mapping of Addresses to Usernames
    #[ink(storage)]
    pub struct Subscrypt {
        start_time: u64,
        pub provider_register_fee: u128,
        // (provider AccountId) -> provider data
        pub providers: HashMap<AccountId, Provider>,
        // (user AccountId) -> user data
        pub users: HashMap<AccountId, User>,
        // (provider AccountId , day_id) -> payment admission
        daily_locked_amounts: HashMap<(AccountId, u64), DailyLockedAmount>,
        // (user AccountId, provider AccountId) -> PlanRecord struct
        pub records: HashMap<(AccountId, AccountId), PlanRecord>,
        // (user AccountId, provider AccountId, plan_index) -> index
        plan_index_to_record_index: HashMap<(AccountId, AccountId, u128), u128>,
        // username -> user AccountId
        username_to_address: HashMap<String, AccountId>,
        // user AccountId -> username
        address_to_username: HashMap<AccountId, String>,
    }

    #[ink(event)]
    pub struct ProviderRegisterEvent {
        #[ink(topic)]
        address: AccountId,
    }

    #[ink(event)]
    pub struct AddPlanEvent {
        #[ink(topic)]
        owner: AccountId,
        duration: u64,
        price: u128,
    }

    #[ink(event)]
    pub struct SubscribeEvent {
        #[ink(topic)]
        provider: AccountId,
        #[ink(topic)]
        plan_index: u128,
        subscription_time: u64,
        duration: u64,
    }
}