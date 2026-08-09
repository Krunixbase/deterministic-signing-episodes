use deterministic_signing_episodes::core::agent::{Episode, EpisodeInput, EpisodeMetadata};
use deterministic_signing_episodes::core::shamir::Share;

/// Deterministic audit consistency test.
/// Ensures that identical episodes produce identical audit records.
/// This is required for Phase I → Phase II → Phase III.
#[test]
fn test_audit_record_is_consistent() {
    let input_1 = EpisodeInput {
        artifact_hash: "artifact_hash_demo".to_string(),
        shares: vec![
            Share { id: 1, value: "A".to_string() },
            Share { id: 2, value: "B".to_string() },
        ],
        metadata: EpisodeMetadata {
            episode_id: "audit-001".to_string(),
            description: "audit consistency test".to_string(),
        },
    };

    let input_2 = EpisodeInput {
        artifact_hash: "artifact_hash_demo".to_string(),
        shares: vec![
            Share { id: 1, value: "A".to_string() },
            Share { id: 2, value: "B".to_string() },
        ],
        metadata: EpisodeMetadata {
            episode_id: "audit-001".to_string(),
            description: "audit consistency test".to_string(),
        },
    };

    let output_1 = Episode::execute(input_1);
    let output_2 = Episode::execute(input_2);

    let audit_1 = output_1.audit_record;
    let audit_2 = output_2.audit_record;

    // Deterministic audit fields
    assert_eq!(audit_1.episode_id, audit_2.episode_id);
    assert_eq!(audit_1.artifact_hash, audit_2.artifact_hash);
    assert_eq!(audit_1.share_count, audit_2.share_count);
    assert_eq!(audit_1.signature_repr, audit_2.signature_repr);
    assert_eq!(audit_1.zeroized, audit_2.zeroized);
}

/// Audit record must fail deterministically when signature is empty.
#[test]
fn test_audit_record_fails_with_empty_signature() {
    use deterministic_signing_episodes::core::audit::create_record;

    let input = EpisodeInput {
        artifact_hash: "artifact_hash_demo".to_string(),
        shares: vec![
            Share { id: 1, value: "A".to_string() },
        ],
        metadata: EpisodeMetadata {
            episode_id: "audit-002".to_string(),
            description: "audit error test".to_string(),
        },
    };

    let result = create_record(&input, "");

    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "cannot create audit record: signature is empty"
    );
}
