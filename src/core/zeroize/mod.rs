/// Deterministic zeroization utilities.
/// Phase I: simple memory wiping.
/// Phase II/III: extended secure zeroization strategies.
pub fn wipe(secret: &mut impl Zeroizable) {
    secret.zeroize();
}

/// Trait for deterministic zeroization.
/// Any ephemeral secret must implement this trait.
pub trait Zeroizable {
    fn zeroize(&mut self);
}

/// Example implementation for the Phase I placeholder secret.
/// Phase II will replace this with a real cryptographic secret type.
impl Zeroizable for crate::core::shamir::Secret {
    fn zeroize(&mut self) {
        // Deterministic zeroization:
        // - overwrite internal representation
        // - no randomness
        // - no environment-dependent behavior
        self.wipe();
    }
}
