#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env, String,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TokenCount,
    TokenOwner(u32),
    TokenUri(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ScholarNFTError {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    NotInitialized = 3,
    TokenNotFound = 4,
    Soulbound = 5,
}

#[contract]
pub struct ScholarNFT;

#[contractimpl]
impl ScholarNFT {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, ScholarNFTError::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenCount, &0_u32);
    }

    pub fn mint(env: Env, to: Address, metadata_uri: String) -> u32 {
        let admin = Self::admin(&env);
        admin.require_auth();

        let next_token_id = env
            .storage()
            .instance()
            .get::<_, u32>(&DataKey::TokenCount)
            .unwrap_or(0)
            + 1;

        env.storage()
            .persistent()
            .set(&DataKey::TokenOwner(next_token_id), &to);
        env.storage()
            .persistent()
            .set(&DataKey::TokenUri(next_token_id), &metadata_uri);
        env.storage()
            .instance()
            .set(&DataKey::TokenCount, &next_token_id);

        next_token_id
    }

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::TokenOwner(token_id))
            .unwrap_or_else(|| panic_with_error!(&env, ScholarNFTError::TokenNotFound))
    }

    pub fn token_uri(env: Env, token_id: u32) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::TokenUri(token_id))
            .unwrap_or_else(|| panic_with_error!(&env, ScholarNFTError::TokenNotFound))
    }

    pub fn transfer(env: Env, _from: Address, _to: Address, _token_id: u32) {
        panic_with_error!(&env, ScholarNFTError::Soulbound)
    }

    fn admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(env, ScholarNFTError::NotInitialized))
    }
}

#[cfg(test)]
mod test;
