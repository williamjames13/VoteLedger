#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, Symbol,
};

use crate::VoteLedgerContract;

mod tests {

    use super::*;

    // Test 1: Happy path
    #[test]
    fn test_vote_success() {
        let env = Env::default();

        let voter = Address::generate(&env);

        let candidate = Symbol::short("A");

        VoteLedgerContract::vote(
            env.clone(),
            voter.clone(),
            candidate.clone(),
        );

        let votes = VoteLedgerContract::get_votes(
            env.clone(),
            candidate,
        );

        assert_eq!(votes, 1);
    }

    // Test 2: Edge case (double vote)
    #[test]
    #[should_panic(expected = "Already voted")]
    fn test_double_vote_fail() {
        let env = Env::default();

        let voter = Address::generate(&env);

        let candidate = Symbol::short("A");

        VoteLedgerContract::vote(
            env.clone(),
            voter.clone(),
            candidate.clone(),
        );

        VoteLedgerContract::vote(
            env.clone(),
            voter,
            candidate,
        );
    }

    // Test 3: State verification
    #[test]
    fn test_has_voted_state() {
        let env = Env::default();

        let voter = Address::generate(&env);

        let candidate = Symbol::short("B");

        VoteLedgerContract::vote(
            env.clone(),
            voter.clone(),
            candidate,
        );

        let voted = VoteLedgerContract::has_voted(
            env.clone(),
            voter,
        );

        assert_eq!(voted, true);
    }

    // Test 4
    #[test]
    fn test_multiple_candidates() {
        let env = Env::default();

        let voter1 = Address::generate(&env);
        let voter2 = Address::generate(&env);

        VoteLedgerContract::vote(
            env.clone(),
            voter1,
            Symbol::short("A"),
        );

        VoteLedgerContract::vote(
            env.clone(),
            voter2,
            Symbol::short("B"),
        );

        assert_eq!(
            VoteLedgerContract::get_votes(
                env.clone(),
                Symbol::short("A")
            ),
            1
        );

        assert_eq!(
            VoteLedgerContract::get_votes(
                env.clone(),
                Symbol::short("B")
            ),
            1
        );
    }

    // Test 5
    #[test]
    fn test_initial_vote_count() {
        let env = Env::default();

        let votes = VoteLedgerContract::get_votes(
            env,
            Symbol::short("X"),
        );

        assert_eq!(votes, 0);
    }
}