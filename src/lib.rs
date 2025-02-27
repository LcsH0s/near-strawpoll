use near_sdk::store::{IterableMap, LookupSet};
use near_sdk::{env, near, require, AccountId, IntoStorageKey, PanicOnDefault};

/// Storage keys for collections to ensure unique prefixes on the blockchain.
#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub enum StorageKey {
    AllowedVoters,
    VotedVoters,
    Candidates,
}

impl IntoStorageKey for StorageKey {
    fn into_storage_key(self) -> Vec<u8> {
        match self {
            Self::AllowedVoters => b"av".to_vec(),
            Self::VotedVoters => b"vv".to_vec(),
            Self::Candidates => b"c".to_vec(),
        }
    }
}

/// The VotingContract structure maintains:
/// - `poll_creator`: The account that created the poll.
/// - `allowed_voters`: A set of account IDs allowed to vote.
/// - `voted_voters`: A set tracking which accounts have already voted.
/// - `candidates`: A map from candidate names to their vote counts.
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct VotingContract {
    poll_creator: AccountId,
    allowed_voters: LookupSet<AccountId>,
    voted_voters: LookupSet<AccountId>,
    candidates: IterableMap<String, u64>,
}

#[near]
impl VotingContract {
    /// Initializes the contract.
    /// * `allowed_voters`: List of accounts allowed to vote.
    /// * `candidates`: List of candidate names.
    ///
    /// The caller becomes the poll creator.
    #[init]
    #[private]
    pub fn new(allowed_voters: Vec<AccountId>, candidates: Vec<String>) -> Self {
        let mut allowed_set = LookupSet::new(StorageKey::AllowedVoters);
        for voter in allowed_voters {
            allowed_set.insert(voter);
        }
        let mut candidates_map = IterableMap::new(StorageKey::Candidates);
        for candidate in candidates {
            candidates_map.insert(candidate, 0);
        }

        Self {
            poll_creator: env::predecessor_account_id(),
            allowed_voters: allowed_set,
            voted_voters: LookupSet::new(StorageKey::VotedVoters),
            candidates: candidates_map,
        }
    }

    /// Casts a vote for the given candidate.
    ///
    /// # Arguments
    ///
    /// * `candidate` - The candidate's name as a string.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not in the allowed voters list,
    /// if they have already voted, or if the candidate does not exist.
    pub fn vote(&mut self, candidate: String) {
        let voter = env::predecessor_account_id();

        // Ensure the caller is allowed to vote.
        require!(
            self.allowed_voters.contains(&voter),
            "You are not allowed to vote"
        );

        // Ensure the caller has not already voted.
        require!(
            !self.voted_voters.contains(&voter),
            "You have already voted"
        );

        // Fetch the current vote count for the candidate.
        let current_votes = self
            .candidates
            .get(&candidate)
            .unwrap_or_else(|| env::panic_str("Candidate does not exist"));

        // Update the vote count.
        self.candidates.insert(candidate, current_votes + 1);

        // Mark the voter as having voted.
        self.voted_voters.insert(voter);
    }

    /// Returns the voting results as a vector of (candidate, vote_count) tuples.
    pub fn get_results(&self) -> Vec<(String, u64)> {
        self.candidates
            .iter()
            .map(|e| (e.0.clone(), e.1.clone()))
            .collect()
    }
}
