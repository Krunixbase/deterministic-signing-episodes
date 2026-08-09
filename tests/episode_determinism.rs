use deterministic_signing_episodes::core::agent::{Episode, EpisodeInput, EpisodeMetadata};
use deterministic_signing_episodes::core::shamir::Share;

/// Deterministic episode test:
/// Ensures that identical inputs produce identical outputs.
/// This is the foundational invariant of Phase I → Phase II → Phase III.
#[test]
fn test_episode_is_deterministic() {
    let input_1 = EpisodeInput {
        artifact_hash: "artifact_hash_demo".to_string(),
        shares: vec![
            Share { id: 1, value: "A".to_string() },
            Share { id: 2, value: "B".to_string() },
        ],
        metadata: EpisodeMetadata {
            episode_id: "ep-001".to_string(),
            description: "deterministic test".to_string(),
        },
    };

    let input_2 = EpisodeInput {
        artifact_hash: "artifact_hash_demo".to_string(),
        shares: vec![
            Share { id: 1, value: "A".to_string() },
            Share { id: 2, value: "B".to_string() },
        ],
        metadata: EpisodeMetadata {
            episode_id: "ep-001".to_string(),
            description: "deterministic test".to_string(),
        },
    };

    let output_1 = Episode::execute(input_1);
    let output_2 = Episode::execute(input_2);

    // Deterministic signature
    assert_eq!(output_1.signature, output_2.signature);

    // Deterministic audit record
    assert_eq!(output_1.audit_record.episode_id, output_2.audit_record.episode_id);
    assert_eq!(output_1.audit_record.artifact_hash, output_2.audit_record.artifact_hash);
    assert_eq!(output_1.audit_record.share_count, output_2.audit_record.share_count);
    assert_eq!(output_1.audit_record.signature_repr, output_2.audit_record.signature_repr);
    assert_eq!(output_1.audit_record.zeroized, output_2.audit_record.zeroized);

    // Deterministic status
    assert!(matches!(output_1.status, deterministic_signing_episodes::core::agent::EpisodeStatus::Success));
    assert!(matches!(output_2.status, deterministic_signing_episodes::core::agent::EpisodeStatus::Success));
}
