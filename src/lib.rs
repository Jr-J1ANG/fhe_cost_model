//! Standalone CKKS ciphertext-multiplication cost model.
//!
//! The first version deliberately exposes a transparent, calibratable model.
//! It produces relative cost units; it does not claim cycle-accurate latency.

pub mod ciphertext;
pub mod config;
pub mod cost;
pub mod fixed_cost;
pub mod immediate_cost;
pub mod lookahead_cost;
pub mod error;
pub mod validation;

pub use ciphertext::CiphertextInfo;
pub use config::{
    CostWeights, ExecutionPolicy, HardwareModelConfig, HardwareProfile, CKKSParams,
};
pub use cost::{estimate_mul_cost, MulCost};
pub use lookahead_cost::CostContext;
pub use fixed_cost::FixedCost;
pub use immediate_cost::{BasisConversionWork, ImmediateCost};
pub use error::ModelError;
