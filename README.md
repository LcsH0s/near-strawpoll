```markdown
# NEAR Voting Contract

A simple smart contract for implementing a decentralized voting poll on the NEAR blockchain. This contract enables a poll creator to set up a voting session with a predefined list of allowed voters and candidates. Each allowed voter can cast one vote for their chosen candidate, and the contract ensures that each voter votes only once.

## Features

- **Restricted Voting:** Only voters included in the allowed voters list can cast a vote.
- **One Vote Per Voter:** The contract tracks voters to ensure they cannot vote more than once.
- **Candidate Tallying:** Maintains vote counts for each candidate and allows results to be queried.
- **Secure Storage:** Uses unique storage prefixes to maintain data integrity on the blockchain.

## Contract Overview

The smart contract is structured with the following key components:

- **poll_creator:** The account that initialized the poll.
- **allowed_voters:** A set of account IDs that are allowed to vote.
- **voted_voters:** A set used to track which voters have already cast their vote.
- **candidates:** A map associating candidate names with their vote counts.

## Functions

### `new(allowed_voters: Vec<AccountId>, candidates: Vec<String>) -> Self`
- **Purpose:** Initializes the contract with the specified list of allowed voters and candidates.
- **Behavior:** The account calling this function becomes the poll creator.

### `vote(candidate: String)`
- **Purpose:** Casts a vote for the specified candidate.
- **Requirements:**
  - The caller must be in the allowed voters list.
  - The caller must not have voted already.
  - The specified candidate must exist.

### `get_results() -> Vec<(String, u64)>`
- **Purpose:** Returns the current vote counts for each candidate as a list of `(candidate, vote_count)` tuples.

## Deployment

To deploy the contract on the NEAR blockchain:

1. **Build the Contract:**

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Deploy Using NEAR CLI:**

   ```bash
   near deploy --wasmFile target/wasm32-unknown-unknown/release/your_contract.wasm --accountId your_account.testnet
   ```


## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [NEAR CLI](https://docs.near.org/tools/near-cli)
- [near-sdk-rs](https://github.com/near/near-sdk-rs)

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.
```
