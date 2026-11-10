use fhe_cost_model::validation::{evaluate, read_csv};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(|| "single_mul_validation.csv".to_string());
    let rows = read_csv(&path)?;
    let metrics = evaluate(&rows);
    println!("samples: {}", metrics.sample_count);
    println!("Kendall tau: {:?}", metrics.kendall_tau);
    println!("pairwise ranking accuracy: {:?}", metrics.pairwise_accuracy);
    println!("best-choice regret: {:?}", metrics.best_choice_regret);
    Ok(())
}
