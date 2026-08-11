use crate::core::shamir;
use crate::core::audit;
use crate::core::zeroize;

/// Deterministic input to a signing episode.
#[derive(Debug, Clone)]
pub struct EpisodeInput {
    pub artifact_hash: String,
    pub shares: Vec<shamir::Share>,
    pub metadata: EpisodeMetadata,
}

/// Explicit, serializable metadata for the episode.
#[derive(Debug, Clone)]
pub struct EpisodeMetadata {
    pub episode_id: String,
    pub description: String,
}

/// Deterministic output of a signing episode.
#[derive(Debug, Clone)]
pub struct EpisodeOutput {
    pub signature: String,
    pub audit_record: audit::AuditRecord,
    pub status: EpisodeStatus,
}

/// Deterministic status of episode execution.
#[derive(Debug, Clone)]
pub enum EpisodeStatus {
    Success,
    ValidationError(String),
    ReconstructionError(String),
    SigningError(String),
    AuditError(String),
}

/// Deterministic signing episode.
pub struct Episode;

impl Episode {
    /// Execute a deterministic signing episode.
    ///
    /// Contract:
    /// - No randomness
    /// - No global state
    /// - No persistent secrets
    /// - Fully auditable, ordered steps
    pub fn execute(input: EpisodeInput) -> EpisodeOutput {
        // 1. Validate inputs
        if let Err(e) = Self::validate_input(&input) {
            return EpisodeOutput {
                signature: String::new(),
                audit_record: audit::AuditRecord::empty(),
                status: EpisodeStatus::ValidationError(e),
            };
        }

        // 2. Reconstruct ephemeral secret (deterministic)
        let mut ephemeral_secret = match shamir::reconstruct(&input.shares) {
            Ok(secret) => secret,
            Err(e) => {
                return EpisodeOutput {
                    signature: String::new(),
                    audit_record: audit::AuditRecord::empty(),
                    status: EpisodeStatus::ReconstructionError(e),
                };
            }
        };

        // 3. Produce deterministic signature
        let signature = match Self::sign(&input.artifact_hash, &ephemeral_secret) {
            Ok(sig) => sig,
            Err(e) => {
                // Zeroize before returning
                zeroize::wipe(&mut ephemeral_secret);
                return EpisodeOutput {
                    signature: String::new(),
                    audit_record: audit::AuditRecord::empty(),
                    status: EpisodeStatus::SigningError(e),
                };
            }
        };

        // 4. Emit deterministic audit record
        let audit_record = match audit::create_record(&input, &signature) {
            Ok(record) => record,
            Err(e) => {
                // Zeroize before returning
                zeroize::wipe(&mut ephemeral_secret);
                return EpisodeOutput {
                    signature,
                    audit_record: audit::AuditRecord::empty(),
                    status: EpisodeStatus::AuditError(e),
                };
            }
        };

        // 5. Zeroize ephemeral secret
        zeroize::wipe(&mut ephemeral_secret);

        // 6. Return deterministic output
        EpisodeOutput {
            signature,
            audit_record,
            status: EpisodeStatus::Success,
        }
    }

    fn validate_input(input: &EpisodeInput) -> Result<(), String> {
        if input.artifact_hash.is_empty() {
            return Err("artifact_hash must not be empty".to_string());
        }
        if input.shares.is_empty() {
            return Err("at least one share is required".to_string());
        }
        if input.metadata.episode_id.is_empty() {
            return Err("episode_id must not be empty".to_string());
        }
        Ok(())
    }

    fn sign(artifact_hash: &str, ephemeral_secret: &shamir::Secret) -> Result<String, String> {
        // Phase I: placeholder deterministic "signature"
        // In Phase II/III this becomes a real deterministic cryptographic signature.
        Ok(format!("sig({}:{})", artifact_hash, ephemeral_secret.as_repr()))
    }
}
