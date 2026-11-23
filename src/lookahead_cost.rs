//! Heuristic lookahead cost for the predicted output level and scale.
//! This is intentionally separate from the paper-reproduced datapath workload.

use crate::{CiphertextInfo, HardwareModelConfig};

#[derive(Clone, Debug, Default)]
pub struct CostContext {
    pub reference_scale_log2: Option<f64>,
    pub reference_level: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct LookaheadCost {
    pub scale_penalty: f64,
    pub level_penalty: f64,
    pub total_cost: f64,
    pub output_scale_log2: f64,
    pub output_level: usize,
}

pub fn estimate_lookahead_cost(
    lhs: &CiphertextInfo,
    rhs: &CiphertextInfo,
    context: &CostContext,
    config: &HardwareModelConfig,
) -> LookaheadCost {
    let rescale_log2 = if config.execution.rescale_after_mul {
        config.execution.rescale_prime_log2
    } else {
        0.0
    };
    let output_scale_log2 = lhs.scale_log2 + rhs.scale_log2 - rescale_log2;
    let aligned_level = lhs.level.min(rhs.level);
    let output_level = aligned_level.saturating_sub(if config.execution.rescale_after_mul { 1 } else { 0 });

    let reference_scale = context.reference_scale_log2
        .unwrap_or(config.weights.reference_scale_log2);
    let reference_level = context.reference_level
        .unwrap_or(config.weights.reference_level);

    let scale_penalty = (output_scale_log2 - reference_scale).abs()
        * config.weights.scale_deviation_weight;
    let level_penalty = output_level.abs_diff(reference_level) as f64
        * config.weights.level_mismatch_weight;

    LookaheadCost {
        scale_penalty,
        level_penalty,
        total_cost: scale_penalty + level_penalty,
        output_scale_log2,
        output_level,
    }
}
