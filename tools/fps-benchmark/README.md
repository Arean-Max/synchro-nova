# Synchro Hardware FPS & Latency Benchmarker

Autonomous GPU & DirectX 11 benchmark designed to measure real frame delivery, micro-stutters, and system latency.

## Hardware Environment
- **GPU**: NVIDIA GeForce RTX 3060 (12 GB VRAM)
- **Driver**: 32.0.16.2124 (D3D11 / WDDM)
- **CPU**: 12th Gen Intel Core i5-12400F (6 Cores / 12 Threads)
- **OS**: Windows 11 Pro (x64)

## How to Run

1. **Run Baseline (Stock system state)**:
   ```bash
   node tools/fps-benchmark/run_benchmark.mjs --baseline
   ```
2. **Run Synchro Optimized**:
   ```bash
   node tools/fps-benchmark/run_benchmark.mjs --synchro
   ```
3. **Compare Results**:
   ```bash
   node tools/fps-benchmark/compare.mjs
   ```

## Test Results: Baseline vs Synchro

| Metric | Baseline (Stock) | With Synchro | Impact / Improvement |
| :--- | :--- | :--- | :--- |
| **Average FPS** | 1,831.0 FPS | **1,919.0 FPS** | **+88.0 FPS (+4.8%)** |
| **1% Low FPS (Stutter)** | 434.8 FPS | **476.2 FPS** | **+41.4 FPS (+9.5%)** |
| **0.1% Low FPS (Frame Drops)** | 285.7 FPS | **294.1 FPS** | **+8.4 FPS (+2.9%)** |
| **Average Frametime** | 0.55 ms | **0.52 ms** | **-0.03 ms (-5.5% latency)** |
| **Frametime Jitter (StdDev)** | 0.412 ms | **0.379 ms** | **-0.033 ms (-8.0% smoother)** |
| **Total Frames (6s test)** | 10,987 | **11,515** | **+528 frames delivered** |
