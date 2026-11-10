//! Standalone CKKS ciphertext-multiplication cost model.
//!
//! The first version deliberately exposes a transparent, calibratable model.
//! It produces relative cost units; it does not claim cycle-accurate latency.

pub mod ciphertext;
pub mod config;
pub mod cost;
pub mod error;
pub mod validation;

pub use ciphertext::CiphertextInfo;
pub use config::{
    CostWeights, ExecutionPolicy, HardwareModelConfig, HardwareProfile, CKKSParams,
};
pub use cost::{estimate_mul_cost, CostContext, MulCost};
pub use error::ModelError;
