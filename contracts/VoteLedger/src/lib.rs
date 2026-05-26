#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol,
};

#[contract]
pub struct VoteLedgerContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    HasVoted(Address),
    Candidate(Symbol),
}

#[contractimpl]
impl VoteLedgerContract {

    // Initialize admin wallet
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Vote for a candidate
    pub fn vote(env: Env, voter: Address, candidate: Symbol) {
        voter.require_auth();

        let voted_key = DataKey::HasVoted(voter.clone());

        // Prevent double voting
        if env.storage().persistent().has(&voted_key) {
            panic!("Already voted");
        }

        // Mark voter as voted
        env.storage().persistent().set(&voted_key, &true);

        // Increment candidate count
        let candidate_key = DataKey::Candidate(candidate.clone());

        let current: u32 = env
            .storage()
            .persistent()
            .get(&candidate_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&candidate_key, &(current + 1));
    }

    // Get candidate vote count
    pub fn get_votes(env: Env, candidate: Symbol) -> u32 {
        let candidate_key = DataKey::Candidate(candidate);

        env.storage()
            .persistent()
            .get(&candidate_key)
            .unwrap_or(0)
    }

    // Check if voter already voted
    pub fn has_voted(env: Env, voter: Address) -> bool {
        let voted_key = DataKey::HasVoted(voter);

        env.storage()
            .persistent()
            .get(&voted_key)
            .unwrap_or(false)
    }
}