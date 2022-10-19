use crate::traits::{
    data::Pool,
    farming::Data,
};
use ink_env::AccountId;
use openbrush::traits::Storage;

#[openbrush::trait_definition]
pub trait FarmingGetters: Storage<Data> {
    #[ink(message)]
    fn pool_length(&self) -> u32 {
        self.data::<Data>().pool_info_length
    }

    #[ink(message)]
    fn get_pool_infos(&self, pool_id: u32) -> Option<Pool> {
        self.data::<Data>().pool_info.get(&pool_id)
    }

    #[ink(message)]
    fn get_user_info(&self, pool_id: u32, user: AccountId) -> Option<(u128, i128)> {
        self.data::<Data>().user_info.get(&(pool_id, user))
    }

    #[ink(message)]
    fn get_lp_token(&self, pool_id: u32) -> Option<AccountId> {
        self.data::<Data>().lp_tokens.get(pool_id as usize).copied()
    }
}
