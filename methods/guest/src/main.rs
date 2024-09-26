use risc0_zkvm::guest::env;
/*
Public Inputs:
Previous State Root (Root1): The Merkle tree root before the state transition.
New State Root (Root2): The Merkle tree root after the state transition.
Blob Data Commitment (blobCommitment): The commitment to the data blob containing the new votes.

Private Inputs:
Votes: The list of new votes being processed.
Nullifiers: The nullifiers associated with the new votes.
Voter Authentication Data: Signatures or zkProofs from voters over their ballots.
Merkle Proofs of Inclusion: Proofs that each voter is included in the census, using the Census Root retrieved from Root1.
Merkle Tree Update Witnesses: Necessary data (e.g., Merkle paths) to update the Merkle tree from Root1 to Root2.
Ballot Validation Data: Information required to validate that votes comply with the ballot rules, using the Ballot Mode retrieved from Root1.
Process Parameters: Retrieved from Root1 within the circuit, such as processId, censusRoot, ballotMode, and originRoot.

The following constraints must be enforced by the circuit.

Immutable Process Parameters: Ensure that critical process parameters (such as `censusRoot`, `ballotMode`, `processId` and `originRoot`) retrieved from `Root1` remain consistent and are not altered in the transition.
Vote Validity: Validate that each vote complies with the ballot rules specified by `ballotMode` (e.g., allowed options, maximum selections).
Voter Authentication: Verify the authenticity of each vote by checking the voter's franchise proof.
Voter Eligibility (Census Verification): Confirm that each voter is included in the census by verifying Merkle proofs of inclusion against the `censusRoot` retrieved from `Root1`.
Nullifier Non-Existence: Ensure that the nullifier for each vote does not exist in the current state (`Root1`), preventing double voting.
Nullifier Addition: Correctly add each new nullifier to the state, resulting in `Root2`, updating the Merkle tree accordingly.
Accurate Results Update: Ensure that the voting results are accurately updated by adding the new votes to the previous results retrieved from `Root1`, so that `results2 = results1 + votes`.
Origin Root Consistency: Verify that the `originRoot` remains unchanged or is correctly set if this is the first state transition.
Blob Data Integrity:  Confirm that the data used in the circuit (votes, nullifiers) corresponds to the `blobCommitment` provided as a public input, ensuring that the votes processed are exactly those published in the data blob.
State Transition Validity: Ensure that the new state root (`Root2`) is correctly computed from `Root1` by applying the validated votes and updates to the Merkle tree.

 */

fn main() {
    // PSEUDOCODE
    let input = env::read();
    type Inputs struct {
        // public inputs
        previous_state_root: Hash, // Root1: The Merkle tree root before the state transition.
        new_state_root: Hash,       // Root2: The Merkle tree root after the state transition.
        blob_commitment: Hash, // blobCommitment: The commitment to the data blob containing the new votes.
        // the rest are private
        votes: []Votes, // The list of new votes being processed.
        nullifiers: []Hash,  // The nullifiers associated with the new votes.
        voter_auths: map[VoteID]VoterAuth,   // Signatures or zkProofs from voters over their ballots.
        merkle_proofs_of_inclusion: []MerkleProof, // Proofs that each voter is included in the census
        merkle_tree_update_witnesses: []MerkleUpdates, // Necessary data (e.g., Merkle paths) to update the Merkle tree from Root1 to Root2.
        ballot_rules: []BallotRule, // Information required to validate that votes comply with the ballot rules,
    }

    for vote in votes {
        let process_params = extract_process_params_from_root(vote.ProcessID, previous_state_root);
        ensure_process_params_remain_consistent(process_params, new_state_root);
        ensure_vote_respects_ballot_rules(vote, ballot_rules);
        verify_vote_auth(voter_auths[vote.id]);
        verify_voter_is_in_census(vote, process_params.census_root);
        add_vote_to_process(process, vote) // ????
    }

    let state_root = previous_state_root;
    for nullifier in nullifiers {
        prevent_double_voting(nullifier, previous_state_root);
        let state_root = add_nullifier_to_state(state_root, nullifier);
    }

    //// TODO ////
    // Origin Root Consistency: Verify that the `originRoot` remains unchanged or is correctly set if this is the first state transition.
    // Blob Data Integrity:  Confirm that the data used in the circuit (votes, nullifiers) corresponds to the `blobCommitment` provided as a public input, ensuring that the votes processed are exactly those published in the data blob.
    // State Transition Validity: Ensure that the new state root (`Root2`) is correctly computed from `Root1` by applying the validated votes and updates to the Merkle tree.

    // write public output to the journal
    env::commit(&b);
}

fn ensure_process_params_remain_consistent(process_params, new_state_root) bool {
    new_pp = extract_process_params_from_root(pid, new_state_root);
    return process_params = new_pp
}

fn extract_process_params_from_root(process_id, state_root) ProcessParams {
    let process = state_root.find_process(process_id)
    return ProcessParams {
        process_id: process.id,
        census_root: process.census_root,
        ballot_mode: process.ballot_mode,
        origin_root: process.origin_root,
    }
}

fn ensure_vote_respects_ballot_rules(vote, ballot_rules) {
    // TBD
}

fn verify_vote_auth(voter_auth) {
    switch voter_auth.type {
        ZK:
            voter_auth.zkproof.verify();
        SIGNATURE:
            // TBD
    }
}
fn verify_voter_is_in_census(vote, process_params.census_root) {
    // TBD
}