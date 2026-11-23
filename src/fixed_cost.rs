//! Fixed cost: the ciphertext-ciphertext polynomial multiplication stage
//! corresponding to Algorithm 1 and the five-Montgomery-unit datapath in Fig. 4.

use serde::{Deserialize, Serialize};
use crate::{CiphertextInfo, HardwareModelConfig, ModelError};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FixedCost {
    /// Four polynomial products: C10*C20, C10*C21, C11*C20, C11*C21.
    pub polynomial_products: u64,
    /// Coefficient-wise Montgomery products across active Q limbs: 4*N*LQ.
    pub montgomery_multiplications: u64,
    /// One polynomial addition for the cross term D1.
    pub polynomial_additions: u64,
    pub total_cost: f64,
}

pub fn estimate_fixed_cost(
    lhs: &CiphertextInfo,
    rhs: &CiphertextInfo,
    config: &HardwareModelConfig,
) -> Result<FixedCost, ModelError> {
    if config.ckks.polynomial_degree == 0 {
        return Err(ModelError::InvalidConfig("polynomial_degree must be non-zero"));
    }
    let n = config.ckks.polynomial_degree as u64;
    let lq = lhs.level.min(rhs.level) as u64;

    // Algorithm 1 assumes two-component inputs. General component counts are
    // deliberately rejected here rather than silently applying the wrong formula.
    if lhs.num_components != 2 || rhs.num_components != 2 {
        return Err(ModelError::InvalidCiphertext(
            "paper-based fixed-cost model currently requires two-component inputs",
        ));
    }

    let polynomial_products = 4;
    let montgomery_multiplications = polynomial_products * n * lq;
    let polynomial_additions = n * lq;

    let hw = &config.hardware;
    let total_cost =
        montgomery_multiplications as f64 * hw.montgomery_mul_cost
        + polynomial_additions as f64 * hw.modular_add_cost;

    Ok(FixedCost {
        polynomial_products,
        montgomery_multiplications,
        polynomial_additions,
        total_cost,
    })
}
