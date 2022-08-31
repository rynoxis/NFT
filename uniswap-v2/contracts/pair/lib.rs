#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pair {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            pausable::*,
            psp22::*,
        },
        traits::Storage,
    };
    use uniswap_v2::{
        impls::pair::*,
        traits::pair::*,
    };

    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount_0: Balance,
        pub amount_1: Balance,
    }

    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount_0: Balance,
        pub amount_1: Balance,
        #[ink(topic)]
        pub to: AccountId,
    }

    #[ink(event)]
    pub struct Swap {
        #[ink(topic)]
        pub sender: AccountId,
        pub amount_0_in: Balance,
        pub amount_1_in: Balance,
        pub amount_0_out: Balance,
        pub amount_1_out: Balance,
        #[ink(topic)]
        pub to: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PairContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pause: pausable::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pair: data::Data,
    }

    impl PSP22 for PairContract {}

    impl Pausable for PairContract {}

    impl Ownable for PairContract {}

    impl Pair for PairContract {
        fn _emit_mint_event(&self, sender: AccountId, amount_0: Balance, amount_1: Balance) {
            self.env().emit_event(Mint {
                sender,
                amount_0,
                amount_1,
            })
        }

        fn _emit_burn_event(
            &self,
            sender: AccountId,
            amount_0: Balance,
            amount_1: Balance,
            to: AccountId,
        ) {
            self.env().emit_event(Burn {
                sender,
                amount_0,
                amount_1,
                to,
            })
        }

        fn _emit_swap_event(
            &self,
            sender: AccountId,
            amount_0_in: Balance,
            amount_1_in: Balance,
            amount_0_out: Balance,
            amount_1_out: Balance,
            to: AccountId,
        ) {
            self.env().emit_event(Swap {
                sender,
                amount_0_in,
                amount_1_in,
                amount_0_out,
                amount_1_out,
                to,
            })
        }
    }

    impl PairContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
            })
        }
    }
}
