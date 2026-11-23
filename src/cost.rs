//! Public cost API: aggregate the three independently modeled cost components.

use serde::{Deserialize, Serialize};

use crate::{CiphertextInfo, HardwareModelConfig, ModelError};
use crate::fixed_cost::{estimate_fixed_cost, FixedCost};
use crate::immediate_cost::{estimate_immediate_cost, ImmediateCost};
use crate::lookahead_cost::estimate_lookahead_cost;

pub use crate::lookahead_cost::CostContext;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MulCost {
    pub fixed_cost: f64,
    pub immediate_cost: f64,
    pub lookahead_cost: f64,
    pub total_cost: f64,
    pub output_scale_log2: f64,
    pub output_level: usize,
    pub aligned_input_level: usize,
    pub fixed_breakdown: FixedCost,
    pub immediate_breakdown: ImmediateCost,
    pub lookahead_breakdown: LookaheadCostSerializable,
}

/// Serializable mirror for the lookahead breakdown.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LookaheadCostSerializable {
    pub scale_penalty: f64,
    pub level_penalty: f64,
    pub total_cost: f64,
}

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
    if config.ckks.q_limb_count == 0 || config.ckks.p_limb_count == 0 {
        return Err(ModelError::InvalidConfig("Q/P limb counts must be non-zero"));
    }
    if lhs.level > config.ckks.q_limb_count || rhs.level > config.ckks.q_limb_count {
        return Err(ModelError::InvalidCiphertext("active Q limb count exceeds configured Q basis"));
    }
    if (!lhs.is_ntt_form) || (!rhs.is_ntt_form) {
        return Err(ModelError::InvalidCiphertext(
            "paper-based multiplier model assumes NTT-form input ciphertexts",
        ));
    }
    if !config.ckks.q_moduli.is_empty() && config.ckks.q_moduli.len() != config.ckks.q_limb_count {
        return Err(ModelError::InvalidConfig("q_moduli length must match q_limb_count"));
    }
    if !config.ckks.p_moduli.is_empty() && config.ckks.p_moduli.len() != config.ckks.p_limb_count {
        return Err(ModelError::InvalidConfig("p_moduli length must match p_limb_count"));
    }
    if config.hardware.ntt_streams == 0
        || config.hardware.modular_switch_lanes == 0
        || config.hardware.montgomery_units == 0
    {
        return Err(ModelError::InvalidConfig("hardware parallelism parameters must be non-zero"));
    }

    let fixed = estimate_fixed_cost(lhs, rhs, config)?;
    let immediate = estimate_immediate_cost(lhs, rhs, config)?;
    let lookahead = estimate_lookahead_cost(lhs, rhs, context, config);

    let fixed_cost = fixed.total_cost;
    let immediate_cost = immediate.total_cost;
    let lookahead_cost = lookahead.total_cost;
    let total_cost = fixed_cost + immediate_cost + lookahead_cost;

    Ok(MulCost {
        fixed_cost,
        immediate_cost,
        lookahead_cost,
        total_cost,
        output_scale_log2: lookahead.output_scale_log2,
        output_level: lookahead.output_level,
        aligned_input_level: lhs.level.min(rhs.level),
        fixed_breakdown: fixed,
        immediate_breakdown: immediate,
        lookahead_breakdown: LookaheadCostSerializable {
            scale_penalty: lookahead.scale_penalty,
            level_penalty: lookahead.level_penalty,
            total_cost: lookahead.total_cost,
        },
    })
}
