//! Immediate cost: relinearization implemented as the three KeySwitch stages
//! in Algorithm 2: modular raising, key multiplication, modular reduction.

use serde::{Deserialize, Serialize};
use crate::{CiphertextInfo, HardwareModelConfig, ModelError};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BasisConversionWork {
    /// Source-basis normalization: one multiplication per source limb/coefficient.
    pub normalization_multiplications: u64,
    /// Cross-basis products: source_limbs * destination_limbs * N.
    pub cross_basis_multiplications: u64,
    /// Accumulation over source limbs in each destination modulus.
    pub accumulation_additions: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImmediateCost {
    // Stage 1: modular raising Q -> P
    pub raising_input_intt_transforms: u64,
    pub q_to_p: BasisConversionWork,
    pub raising_output_ntt_transforms: u64,

    // Stage 2: multiply by the two-polynomial relinearization key
    pub key_multiplications_q_basis: u64,
    pub key_multiplications_p_basis: u64,
    pub key_product_intt_transforms: u64,

    // Stage 3: modular reduction P -> Q
    pub reduction_input_intt_transforms: u64,
    pub p_to_q: BasisConversionWork,
    pub reduction_output_ntt_transforms: u64,
    pub reduction_subtractions: u64,
    pub p_inverse_multiplications: u64,

    pub total_ntt_intt_transforms: u64,
    pub total_montgomery_multiplications: u64,
    pub total_modular_additions: u64,
    pub total_cost: f64,
}

fn basis_conversion_work(n: u64, source_limbs: u64, destination_limbs: u64) -> BasisConversionWork {
    BasisConversionWork {
        normalization_multiplications: n * source_limbs,
        cross_basis_multiplications: n * source_limbs * destination_limbs,
        accumulation_additions: n * destination_limbs * source_limbs.saturating_sub(1),
    }
}

pub fn estimate_immediate_cost(
    lhs: &CiphertextInfo,
    rhs: &CiphertextInfo,
    config: &HardwareModelConfig,
) -> Result<ImmediateCost, ModelError> {
    if !config.execution.relinearize_after_mul {
        return Ok(ImmediateCost::default());
    }
    let n = config.ckks.polynomial_degree as u64;
    let lq = lhs.level.min(rhs.level) as u64;
    let lp = config.ckks.p_limb_count as u64;
    if n == 0 || lq == 0 || lp == 0 {
        return Err(ModelError::InvalidConfig(
            "polynomial degree, active Q limbs, and P limb count must be non-zero for relinearization",
        ));
    }

    // Algorithm 2 key-switches the high-degree product D2. Its input is one
    // polynomial; the relinearization key has two output components.
    let q_to_p = basis_conversion_work(n, lq, lp);
    let p_to_q = basis_conversion_work(n, lp, lq);

    let raising_input_intt_transforms = 1;
    let raising_output_ntt_transforms = lp;
    let key_multiplications_q_basis = 2 * n * lq;
    let key_multiplications_p_basis = 2 * n * lp;
    let key_product_intt_transforms = 2 * lp;
    let reduction_input_intt_transforms = 0; // already counted as key_product_intt_transforms
    let reduction_output_ntt_transforms = 2 * lq;
    let reduction_subtractions = 2 * n * lq;
    let p_inverse_multiplications = 2 * n * lq;

    let total_ntt_intt_transforms =
        raising_input_intt_transforms + raising_output_ntt_transforms
        + key_product_intt_transforms + reduction_input_intt_transforms
        + reduction_output_ntt_transforms;

    let total_montgomery_multiplications =
        q_to_p.normalization_multiplications + q_to_p.cross_basis_multiplications
        + key_multiplications_q_basis + key_multiplications_p_basis
        + p_to_q.normalization_multiplications + p_to_q.cross_basis_multiplications
        + p_inverse_multiplications;

    let total_modular_additions =
        q_to_p.accumulation_additions + p_to_q.accumulation_additions
        + reduction_subtractions;

    let hw = &config.hardware;
    let total_cost =
        total_ntt_intt_transforms as f64 * hw.ntt_transform_cost
        + total_montgomery_multiplications as f64 * hw.montgomery_mul_cost
        + total_modular_additions as f64 * hw.modular_add_cost;

    Ok(ImmediateCost {
        raising_input_intt_transforms,
        q_to_p,
        raising_output_ntt_transforms,
        key_multiplications_q_basis,
        key_multiplications_p_basis,
        key_product_intt_transforms,
        reduction_input_intt_transforms,
        p_to_q,
        reduction_output_ntt_transforms,
        reduction_subtractions,
        p_inverse_multiplications,
        total_ntt_intt_transforms,
        total_montgomery_multiplications,
        total_modular_additions,
        total_cost,
    })
}
