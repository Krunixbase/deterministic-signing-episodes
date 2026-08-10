//! Deterministic Ethereum Signing Example
//!
//! This example demonstrates how a deterministic signing episode
//! can be used to sign an Ethereum transaction without randomness,
//! producing a reproducible audit record and allowing deterministic replay.

use deterministic_signing_episodes::{
    agent::episode::DeterministicEpisode,
    audit::AuditRecord,
    shamir::DeterministicReconstruction,
};

use ethers_core::types::{TransactionRequest, Signature};
use ethers_core::utils::keccak256;

fn main() {
    // ------------------------------------------------------------
    // 1. Prepare a sample Ethereum transaction
    // ------------------------------------------------------------
    let tx = TransactionRequest {
        from: None,
        to: Some("0x000000000000000000000000000000000000dead".parse().unwrap()),
        value: Some(1_000_000_000_000u64.into()),
        gas: Some(21_000u64.into()),
        gas_price: Some(1_000_000_000u64.into()),
        nonce: Some(0u64.into()),
        data: None,
        ..Default::default()
    };

    // Serialize transaction (RLP)
    let encoded_tx = tx.rlp();
    let tx_hash = keccak256(&encoded_tx);

    println!("Ethereum TX hash: 0x{}", hex::encode(tx_hash));

    // ------------------------------------------------------------
    // 2. Create deterministic signing episode
    // ------------------------------------------------------------
    let mut episode = DeterministicEpisode::new("eth_signing_example");

    // Deterministic ephemeral key generation (no randomness)
    let ephemeral_key = episode.generate_ephemeral_key();

    // ------------------------------------------------------------
    // 3. Deterministic signing of the transaction hash
    // ------------------------------------------------------------
    let signature: Signature = episode.sign_hash(&tx_hash, &ephemeral_key);

    println!("Deterministic signature:");
    println!("  r = 0x{}", hex::encode(signature.r));
    println!("  s = 0x{}", hex::encode(signature.s));
    println!("  v = {}", signature.v);

    // ------------------------------------------------------------
    // 4. Produce deterministic audit record
    // ------------------------------------------------------------
    let audit = AuditRecord::new(
        "eth_signing_example",
        &tx_hash,
        &signature,
        &ephemeral_key,
    );

    println!("\nAudit record (deterministic):");
    println!("{}", audit.to_json_pretty());

    // ------------------------------------------------------------
    // 5. Deterministic replay (verification)
    // ------------------------------------------------------------
    let reconstructed_key =
        DeterministicReconstruction::reconstruct(&audit);

    let replay_signature =
        DeterministicEpisode::replay(&tx_hash, &reconstructed_key);

    println!("\nReplay signature matches original: {}", replay_signature == signature);
}
