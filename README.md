# fhe_cost_model

A standalone Rust library for estimating **relative CKKS ciphertext multiplication cost**. It is intentionally independent of `egg`, `egg_local`, OpenFHE, and any optimizer.

> **Important:** the default hardware coefficients are placeholders. This crate is an architecture and experiment scaffold, not a validated latency predictor yet. Do not interpret its default output as measured hardware latency.

## Architecture

- `ciphertext.rs`: ciphertext state (`level`, `scale_log2`, active Q moduli, component count, NTT form).
- `config.rs`: CKKS parameters, hardware profile, execution policy, and model weights.
- `cost.rs`: fixed + immediate + lookahead cost decomposition and the public estimator.
- `validation.rs`: CSV schema and ranking/regret metrics for measured single-multiplication experiments.

The first formula is intentionally a transparent prototype: its operation-count coefficients and scale/level penalties must be calibrated and experimentally checked. In particular, scale deviation is a heuristic lookahead penalty, not a physical operation or measured latency. The crate has no dependency on e-graph concepts. The optimizer should estimate/choose the two input `CiphertextInfo` values and pass them to this library.

## Build and smoke test

```bash
cargo test
cargo run --example single_mul_sweep
```

The sweep writes `single_mul_predictions.csv`. It contains predictions only; it does **not** measure OpenFHE or hardware latency.

## Validate against measured latency

Create `single_mul_validation.csv` with these columns:

```csv
case_id,level_a,level_b,scale_a_log2,scale_b_log2,predicted_cost,measured_time_ns
case_0,4,4,40,40,123.4,67000
```

Use the predicted costs from the model and measured timings from the benchmark. Then run:

```bash
cargo run --example evaluate_validation -- single_mul_validation.csv
```

Metrics: Kendall tau, pairwise ranking accuracy, and best-choice regret. Use multiple repetitions per case and put the median measured latency in the CSV. Keep raw repetitions separately for traceability.

## Suggested first experiment

1. Fix polynomial degree, Q/P chains, OpenFHE version, scaling technique, thread count, and hardware.
2. Benchmark `MulNoRelin`, `RelinOnly`, and `Mul+Relin` separately where the API permits.
3. Sweep input level pairs `(4,4)`, `(4,3)`, `(4,2)`, `(3,2)`.
4. Sweep scale pairs such as `(40,40)`, `(35,40)`, `(40,35)` in log2 scale units. Confirm all cases are semantically valid under the selected OpenFHE scaling technique.
5. Measure rescale and mod-switch separately, so the model does not accidentally charge the same operation twice.
6. Calibrate hardware-profile coefficients on a training subset; report ranking metrics on held-out cases. Compare fixed-only, fixed+immediate, and fixed+immediate+lookahead ablations.

OpenFHE wall-clock timings validate software-backend trends, not FPGA/ASIC latency. For hardware-specific accuracy claims, use measurements from the target accelerator or published module-level measurements.

## Integration from another local crate

In the optimizer's `Cargo.toml`:

```toml
[dependencies]
fhe_cost_model = { path = "../fhe_cost_model" }
```

Example:

```rust
use fhe_cost_model::{estimate_mul_cost, CiphertextInfo, CostContext, HardwareModelConfig};

let config = HardwareModelConfig::default();
let lhs = CiphertextInfo {
    level: 4,
    scale_log2: 40.0,
    active_q_moduli: vec![],
    num_components: 2,
    is_ntt_form: true,
};
let rhs = lhs.clone();
let cost = estimate_mul_cost(&lhs, &rhs, &CostContext::default(), &config)?;
println!("relative cost = {}", cost.total_cost);
```

## Publish to GitHub

Create an empty GitHub repository named `fhe_cost_model`, then from this directory:

```bash
git init
git add .
git commit -m "Initial fhe_cost_model crate"
git branch -M main
git remote add origin https://github.com/<YOUR_GITHUB_USERNAME>/fhe_cost_model.git
git push -u origin main
```

Do not commit local measurement outputs unless you intentionally want them versioned; `*.csv` is ignored by default. Consider committing a small anonymized/reference dataset under `validation/data/` later if reproducibility is important.
