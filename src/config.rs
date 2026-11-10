use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CKKSParams {
    pub polynomial_degree: usize,
    pub q_moduli: Vec<u64>,
    pub p_moduli: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// Relative cost of one NTT at the configured polynomial degree.
    pub ntt_cost: f64,
    /// Relative cost of one modular multiplication unit.
    pub modular_mul_cost: f64,
    /// Relative cost of one unit of RNS basis conversion.
    pub basis_conversion_cost: f64,
    /// Relative cost of one CKKS rescale operation.
    pub rescale_cost: f64,
    /// Relative cost of dropping/alignment by one Q limb.
    pub mod_switch_per_limb_cost: f64,
    /// Approximate NTT count per active Q limb for ciphertext multiplication.
    pub mul_ntt_per_limb: f64,
    /// Approximate NTT count per active Q limb for relinearization/key switching.
    pub relin_ntt_per_limb: f64,
    /// Approximate modular-multiplication count per active Q limb.
    pub modular_mul_per_limb: f64,
    /// Approximate basis-conversion work per active Q limb.
    pub basis_conversion_per_limb: f64,
}

impl Default for HardwareProfile {
    fn default() -> Self {
        // Neutral starting coefficients only. Calibrate these using measurements.
        Self {
            ntt_cost: 1.0,
            modular_mul_cost: 1.0,
            basis_conversion_cost: 1.0,
            rescale_cost: 1.0,
            mod_switch_per_limb_cost: 1.0,
            mul_ntt_per_limb: 3.0,
            relin_ntt_per_limb: 4.0,
            modular_mul_per_limb: 8.0,
            basis_conversion_per_limb: 2.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub relinearize_after_mul: bool,
    pub rescale_after_mul: bool,
    pub align_levels_before_mul: bool,
    /// log2 of the modulus prime dropped by rescale; used to estimate output scale.
    pub rescale_prime_log2: f64,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            relinearize_after_mul: true,
            rescale_after_mul: true,
            align_levels_before_mul: true,
            rescale_prime_log2: 40.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostWeights {
    /// Weight of scale deviation from the reference scale in lookahead penalty.
    pub scale_deviation_weight: f64,
    /// Weight of predicted future level-alignment work.
    pub level_mismatch_weight: f64,
    pub reference_scale_log2: f64,
    pub reference_level: usize,
}

impl Default for CostWeights {
    fn default() -> Self {
        Self {
            scale_deviation_weight: 0.05,
            level_mismatch_weight: 0.25,
            reference_scale_log2: 40.0,
            reference_level: 4,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareModelConfig {
    pub ckks: CKKSParams,
    pub hardware: HardwareProfile,
    pub execution: ExecutionPolicy,
    pub weights: CostWeights,
}

impl Default for HardwareModelConfig {
    fn default() -> Self {
        Self {
            ckks: CKKSParams {
                polynomial_degree: 1 << 14,
                q_moduli: vec![],
                p_moduli: vec![],
            },
            hardware: HardwareProfile::default(),
            execution: ExecutionPolicy::default(),
            weights: CostWeights::default(),
        }
    }
}
