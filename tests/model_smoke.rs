use fhe_cost_model::{
    estimate_mul_cost, CiphertextInfo, CostContext, HardwareModelConfig,
};

fn ct(level: usize, scale_log2: f64) -> CiphertextInfo {
    CiphertextInfo {
        level,
        scale_log2,
        active_q_moduli: vec![],
        num_components: 2,
        is_ntt_form: true,
    }
}

#[test]
fn estimates_a_valid_multiplication_and_exposes_breakdown() {
    let config = HardwareModelConfig::default();
    let result = estimate_mul_cost(&ct(4, 40.0), &ct(4, 40.0), &CostContext::default(), &config)
        .unwrap();

    assert!(result.total_cost > 0.0);
    assert_eq!(result.fixed_breakdown.polynomial_products, 4);
    assert_eq!(result.fixed_breakdown.polynomial_additions, (1 << 14) * 4);
    assert_eq!(result.immediate_breakdown.raising_output_ntt_transforms, 4);
    assert_eq!(result.immediate_breakdown.key_product_intt_transforms, 8);
    assert_eq!(result.immediate_breakdown.reduction_output_ntt_transforms, 8);
    assert_eq!(
        result.total_cost,
        result.fixed_cost + result.immediate_cost + result.lookahead_cost
    );
}

#[test]
fn rejects_inconsistent_ciphertext_state() {
    let config = HardwareModelConfig::default();
    let mut invalid = ct(4, 40.0);
    invalid.active_q_moduli = vec![17, 19];
    assert!(estimate_mul_cost(&invalid, &ct(4, 40.0), &CostContext::default(), &config).is_err());
}

#[test]
fn q_and_p_limb_counts_are_parameterized() {
    let mut config = HardwareModelConfig::default();
    config.ckks.p_limb_count = 3;
    let result = estimate_mul_cost(&ct(4, 40.0), &ct(4, 40.0), &CostContext::default(), &config)
        .unwrap();
    assert_eq!(result.immediate_breakdown.raising_output_ntt_transforms, 3);
    assert_eq!(
        result.immediate_breakdown.q_to_p.cross_basis_multiplications,
        (1 << 14) * 4 * 3
    );
}

#[test]
fn rejects_non_two_component_inputs_for_paper_algorithm1_model() {
    let config = HardwareModelConfig::default();
    let mut ct3 = ct(4, 40.0);
    ct3.num_components = 3;
    assert!(estimate_mul_cost(&ct3, &ct(4, 40.0), &CostContext::default(), &config).is_err());
}
