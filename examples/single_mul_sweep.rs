//! Emits a prediction-only CSV for a small level/scale sweep.
//! Replace/augment this with measured OpenFHE timings before evaluating accuracy.
use fhe_cost_model::{estimate_mul_cost, CiphertextInfo, CostContext, HardwareModelConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = HardwareModelConfig::default();
    let mut w = csv::Writer::from_path("single_mul_predictions.csv")?;
    w.write_record(["case_id", "level_a", "level_b", "scale_a_log2", "scale_b_log2", "predicted_cost"])?;
    let cases = [(4,4,40.0,40.0), (4,3,40.0,40.0), (4,2,40.0,40.0), (3,2,40.0,40.0), (4,4,35.0,40.0), (4,4,40.0,35.0)];
    for (i, (la, lb, sa, sb)) in cases.into_iter().enumerate() {
        let lhs = CiphertextInfo { level: la, scale_log2: sa, active_q_moduli: vec![], num_components: 2, is_ntt_form: true };
        let rhs = CiphertextInfo { level: lb, scale_log2: sb, active_q_moduli: vec![], num_components: 2, is_ntt_form: true };
        let result = estimate_mul_cost(&lhs, &rhs, &CostContext::default(), &config)?;
        w.write_record([format!("case_{i}"), la.to_string(), lb.to_string(), sa.to_string(), sb.to_string(), result.total_cost.to_string()])?;
    }
    w.flush()?;
    println!("Wrote single_mul_predictions.csv (prediction only; no measured timings yet).");
    Ok(())
}
