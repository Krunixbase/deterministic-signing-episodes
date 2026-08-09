use deterministic_signing_episodes::core::shamir::{Share, reconstruct};

/// Deterministic Shamir reconstruction test.
/// Ensures that identical share sets produce identical reconstructed secrets.
/// This is a foundational invariant for Phase I → Phase II.
#[test]
fn test_shamir_reconstruction_is_deterministic() {
    let shares_1 = vec![
        Share { id: 1, value: "A".to_string() },
        Share { id: 2, value: "B".to_string() },
        Share { id: 3, value: "C".to_string() },
    ];

    let shares_2 = vec![
        Share { id: 1, value: "A".to_string() },
        Share { id: 2, value: "B".to_string() },
        Share { id: 3, value: "C".to_string() },
    ];

    let secret_1 = reconstruct(&shares_1).expect("reconstruction failed");
    let secret_2 = reconstruct(&shares_2).expect("reconstruction failed");

    // Deterministic representation must match exactly
    assert_eq!(secret_1.as_repr(), secret_2.as_repr());
}

/// Reconstruction must fail deterministically when no shares are provided.
#[test]
fn test_shamir_reconstruction_fails_with_no_shares() {
    let shares: Vec<Share> = vec![];

    let result = reconstruct(&shares);

    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "cannot reconstruct secret: no shares provided"
    );
}
