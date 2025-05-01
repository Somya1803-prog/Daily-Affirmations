#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, Vec, Symbol, log};

#[contracttype]
pub enum AffirmationKey {
    Entries(Address),
}

#[contract]
pub struct DailyAffirmations;

#[contractimpl]
impl DailyAffirmations {
    // Add a motivational quote
    pub fn add_affirmation(env: Env, user: Address, affirmation: String) {
        user.require_auth();

        let key = AffirmationKey::Entries(user.clone());
        let mut list: Vec<String> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        list.push_back(affirmation.clone());
        env.storage().persistent().set(&key, &list);

        log!(&env, "Affirmation added by {}: {}", user, affirmation);
    }

    // Retrieve all affirmations by a user
    pub fn get_affirmations(env: Env, user: Address) -> Vec<String> {
        let key = AffirmationKey::Entries(user);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }

    // Get total number of affirmations by a user
    pub fn count_affirmations(env: Env, user: Address) -> u32 {
        let affirmations = Self::get_affirmations(env, user);
        affirmations.len()
    }
}
