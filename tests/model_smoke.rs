use fhe_cost_model::{estimate_mul_cost, CiphertextInfo, CostContext, HardwareModelConfig};

fn ct(level: usize, scale_log2: f64) -> CiphertextInfo {
    CiphertextInfo { level, scale_log2, active_q_moduli: vec![], num_components: 2, is_ntt_form: true }
}

#[test]
fn estimates_a_valid_multiplication() {
    let config = HardwareModelConfig::default();
    let result = estimate_mul_cost(&ct(4, 40.0), &ct(4, 40.0), &CostContext::default(), &config);
    assert!(result.is_ok());
    assert!(result.unwrap().total_cost > 0.0);
}

#[test]
fn rejects_inconsistent_ciphertext_state() {
    let config = HardwareModelConfig::default();
    let mut invalid = ct(4, 40.0);
    invalid.active_q_moduli = vec![17, 19];
    assert!(estimate_mul_cost(&invalid, &ct(4, 40.0), &CostContext::default(), &config).is_err());
}
