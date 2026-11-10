use serde::{Deserialize, Serialize};

/// State metadata required by the first CKKS cost-model version.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CiphertextInfo {
    /// Number of active Q limbs. This is used as the model's level indicator.
    pub level: usize,
    /// log2(scale), avoiding numerical instability from storing large scales.
    pub scale_log2: f64,
    /// Active Q moduli, ordered as in the CKKS modulus chain.
    pub active_q_moduli: Vec<u64>,
    /// Number of ciphertext polynomial components (normally 2 before relin).
    pub num_components: usize,
    pub is_ntt_form: bool,
}

impl CiphertextInfo {
    pub fn validate(&self) -> bool {
        self.level > 0
            && self.num_components >= 2
            && self.scale_log2.is_finite()
            && (self.active_q_moduli.is_empty() || self.active_q_moduli.len() == self.level)
    }
}
