/// Deterministic audit record for a signing episode.
/// Phase I: minimal deterministic structure.
/// Phase II/III: extended distributed metadata.
#[derive(Debug, Clone)]
pub struct AuditRecord {
    pub episode_id: String,
    pub artifact_hash: String,
    pub share_count: usize,
    pub signature_repr: String,
    pub zeroized: bool,
}

impl AuditRecord {
    /// Create an empty audit record (used for error cases).
    pub fn empty() -> Self {
        AuditRecord {
            episode_id: String::new(),
            artifact_hash: String::new(),
            share_count: 0,
            signature_repr: String::new(),
            zeroized: false,
        }
    }
}

/// Deterministic audit creation.
/// Phase I: minimal deterministic fields.
/// Phase II: extended metadata for distributed trust.
/// Phase III: cryptographically bindable audit records.
pub fn create_record(
    input: &crate::core::agent::EpisodeInput,
    signature: &str,
) -> Result<AuditRecord, String> {
    // Deterministic audit record:
    // - no randomness
    // - no timestamps (Phase I)
    // - no environment-dependent fields
    // - strict ordering of fields

    if signature.is_empty() {
        return Err("cannot create audit record: signature is empty".to_string());
    }

    Ok(AuditRecord {
        episode_id: input.metadata.episode_id.clone(),
        artifact_hash: input.artifact_hash.clone(),
        share_count: input.shares.len(),
        signature_repr: signature.to_string(),
        zeroized: true, // Episode guarantees zeroization before returning
    })
}
