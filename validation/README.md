# Single-multiplication validation

This directory is for the measured dataset and benchmark notes. The crate currently provides prediction and metric plumbing; it does not invoke OpenFHE itself.

## Recommended protocol

- Fix the OpenFHE version, CKKS parameters, scaling technique, thread count, and machine.
- Construct valid ciphertexts at the required levels/scales.
- Warm up each case, then collect at least 100 timed repetitions; report median and dispersion.
- Measure multiplication without relinearization, relinearization, and combined multiplication plus relinearization separately if the selected OpenFHE API allows it.
- Measure rescale and level alignment separately. Avoid charging an operation both as part of a combined measurement and again as an immediate cost.
- Keep raw samples in a separate file; summarize each case into the schema in `measurement_schema.csv`.

`measured_time_ns` is the measured operation or explicitly defined operation sequence. Keep the sequence identical across cases. A software OpenFHE benchmark validates software backend trends; it does not establish FPGA/ASIC latency accuracy.
