use serde::{Deserialize, Serialize};

use crate::{CiphertextInfo, HardwareModelConfig, ModelError};

#[derive(Clone, Debug, Default)]
pub struct CostContext {
    /// Optional expected scale at the next consumer / local-scope reference point.
    pub reference_scale_log2: Option<f64>,
    /// Optional expected level at the next consumer / local-scope reference point.
    pub reference_level: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MulCost {
    pub fixed_cost: f64,
    pub immediate_cost: f64,
    pub lookahead_cost: f64,
    pub total_cost: f64,
    pub output_scale_log2: f64,
    pub output_level: usize,
    pub aligned_input_level: usize,
}

/// Estimate a single ciphertext-ciphertext multiplication cost in relative units.
///
/// The coefficients in `HardwareProfile` are deliberately exposed for calibration.
/// The default values are placeholders, not measured hardware constants.
pub fn estimate_mul_cost(
    lhs: &CiphertextInfo,
    rhs: &CiphertextInfo,
    context: &CostContext,
    config: &HardwareModelConfig,
) -> Result<MulCost, ModelError> {
    if !lhs.validate() || !rhs.validate() {
        return Err(ModelError::InvalidCiphertext("state fields are inconsistent"));
    }
    if config.ckks.polynomial_degree == 0 {
        return Err(ModelError::InvalidConfig("polynomial_degree must be non-zero"));
    }

    let hw = &config.hardware;
    let active_limbs = lhs.level.min(rhs.level);
    let level_gap = lhs.level.abs_diff(rhs.level);
    let components = (lhs.num_components * rhs.num_components).max(1) as f64;

    let mul_ntts = hw.mul_ntt_per_limb * active_limbs as f64;
    let relin_ntts = if config.execution.relinearize_after_mul {
        hw.relin_ntt_per_limb * active_limbs as f64
    } else { 0.0 };
    let modular_muls = hw.modular_mul_per_limb * active_limbs as f64 * components.sqrt();
    let basis_work = if config.execution.relinearize_after_mul {
        hw.basis_conversion_per_limb * active_limbs as f64
    } else { 0.0 };

    let fixed_cost = (mul_ntts + relin_ntts) * hw.ntt_cost
        + modular_muls * hw.modular_mul_cost
        + basis_work * hw.basis_conversion_cost;

    let immediate_cost = if config.execution.align_levels_before_mul {
        level_gap as f64 * hw.mod_switch_per_limb_cost
    } else { 0.0 }
        + if config.execution.rescale_after_mul { hw.rescale_cost } else { 0.0 };

    let output_scale_log2 = lhs.scale_log2 + rhs.scale_log2
        - if config.execution.rescale_after_mul { config.execution.rescale_prime_log2 } else { 0.0 };
    let output_level = active_limbs.saturating_sub(if config.execution.rescale_after_mul { 1 } else { 0 });

    let reference_scale = context.reference_scale_log2.unwrap_or(config.weights.reference_scale_log2);
    let reference_level = context.reference_level.unwrap_or(config.weights.reference_level);
    let scale_penalty = (output_scale_log2 - reference_scale).abs()
        * config.weights.scale_deviation_weight;
    let level_penalty = output_level.abs_diff(reference_level) as f64
        * config.weights.level_mismatch_weight;
    let lookahead_cost = scale_penalty + level_penalty;
    let total_cost = fixed_cost + immediate_cost + lookahead_cost;

    Ok(MulCost {
        fixed_cost,
        immediate_cost,
        lookahead_cost,
        total_cost,
        output_scale_log2,
        output_level,
        aligned_input_level: active_limbs,
    })
}
