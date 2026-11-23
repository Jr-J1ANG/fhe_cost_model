use serde::{Deserialize, Serialize};

/// CKKS cryptographic parameters. q_moduli/p_moduli contain the actual primes
/// when known; the explicit limb counts allow architecture studies before all
/// concrete primes have been selected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CKKSParams {
    pub polynomial_degree: usize,
    pub q_moduli: Vec<u64>,
    pub p_moduli: Vec<u64>,
    /// Total Q-basis limb count. Active Q limbs are taken from CiphertextInfo.
    pub q_limb_count: usize,
    /// P-basis limb count used by the key-switching architecture.
    pub p_limb_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// Number of parallel NTT/INTT data streams (paper: n_NTT = 8).
    pub ntt_streams: usize,
    /// Number of parallel modular-switching lanes (paper: n_lane = 2).
    pub modular_switch_lanes: usize,
    /// Number of Montgomery multiplier units in the ciphertext multiplier (paper: 5).
    pub montgomery_units: usize,
    /// Pipeline latency of one Montgomery multiplier, in cycles (paper: 7).
    pub montgomery_latency_cycles: usize,
    /// Relative weights used to turn operation counts into a scalar score.
    /// These are calibration parameters, not measured universal constants.
    pub ntt_transform_cost: f64,
    pub montgomery_mul_cost: f64,
    pub modular_add_cost: f64,
    pub memory_word_cost: f64,
    /// Optional clock frequency for reporting cycle/time estimates.
    pub clock_mhz: f64,
}

impl Default for HardwareProfile {
    fn default() -> Self {
        Self {
            ntt_streams: 8,
            modular_switch_lanes: 2,
            montgomery_units: 5,
            montgomery_latency_cycles: 7,
            ntt_transform_cost: 1.0,
            montgomery_mul_cost: 1.0,
            modular_add_cost: 1.0,
            memory_word_cost: 0.0,
            clock_mhz: 222.0,
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
    pub scale_deviation_weight: f64,
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
                q_limb_count: 4,
                p_limb_count: 4,
            },
            hardware: HardwareProfile::default(),
            execution: ExecutionPolicy::default(),
            weights: CostWeights::default(),
        }
    }
}
