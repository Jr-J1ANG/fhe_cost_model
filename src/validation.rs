use serde::{Deserialize, Serialize};

/// One row in the single-operation validation dataset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationRecord {
    pub case_id: String,
    pub level_a: usize,
    pub level_b: usize,
    pub scale_a_log2: f64,
    pub scale_b_log2: f64,
    pub predicted_cost: f64,
    /// Measured wall-clock latency in nanoseconds, supplied by an external benchmark.
    pub measured_time_ns: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidationMetrics {
    pub sample_count: usize,
    pub kendall_tau: Option<f64>,
    pub pairwise_accuracy: Option<f64>,
    pub best_choice_regret: Option<f64>,
}

pub fn read_csv(path: &str) -> Result<Vec<ValidationRecord>, csv::Error> {
    csv::Reader::from_path(path)?.deserialize().collect()
}

/// Ranking metrics for predicted cost versus measured latency (lower is better).
pub fn evaluate(records: &[ValidationRecord]) -> ValidationMetrics {
    let n = records.len();
    if n < 2 {
        return ValidationMetrics { sample_count: n, kendall_tau: None, pairwise_accuracy: None, best_choice_regret: None };
    }
    let mut concordant = 0.0;
    let mut discordant = 0.0;
    let mut ties = 0.0;
    let mut correct = 0.0;
    let mut comparable = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let dp = (records[i].predicted_cost - records[j].predicted_cost).signum();
            let dm = (records[i].measured_time_ns - records[j].measured_time_ns).signum();
            if dp == 0.0 || dm == 0.0 { ties += 1.0; continue; }
            comparable += 1.0;
            if dp == dm { concordant += 1.0; correct += 1.0; } else { discordant += 1.0; }
        }
    }
    let denom = concordant + discordant + ties;
    let tau = if denom > 0.0 { Some((concordant - discordant) / denom) } else { None };
    let pairwise_accuracy = if comparable > 0.0 { Some(correct / comparable) } else { None };
    let predicted_best = records.iter().min_by(|a,b| a.predicted_cost.total_cmp(&b.predicted_cost)).unwrap();
    let actual_best = records.iter().map(|r| r.measured_time_ns).fold(f64::INFINITY, f64::min);
    let regret = if actual_best > 0.0 { Some((predicted_best.measured_time_ns - actual_best) / actual_best) } else { None };
    ValidationMetrics { sample_count: n, kendall_tau: tau, pairwise_accuracy, best_choice_regret: regret }
}
