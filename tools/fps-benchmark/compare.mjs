import fs from "node:fs";
import path from "node:path";

const dir = path.resolve("C:\\Users\\tyler\\Desktop\\synchro\\tools\\fps-benchmark");
const baselinePath = path.join(dir, "results_baseline.json");
const synchroPath = path.join(dir, "results_synchro.json");

if (!fs.existsSync(baselinePath) || !fs.existsSync(synchroPath)) {
  console.error("Missing baseline or synchro result files.");
  process.exit(1);
}

const base = JSON.parse(fs.readFileSync(baselinePath, "utf8"));
const sync = JSON.parse(fs.readFileSync(synchroPath, "utf8"));

const fpsDiff = (sync.avgFps - base.avgFps).toFixed(1);
const fpsPct = (((sync.avgFps - base.avgFps) / base.avgFps) * 100).toFixed(1);

const oneLowDiff = (sync.onePercentLowFps - base.onePercentLowFps).toFixed(1);
const oneLowPct = (((sync.onePercentLowFps - base.onePercentLowFps) / base.onePercentLowFps) * 100).toFixed(1);

const pointOneLowDiff = (sync.zeroPointOnePercentLowFps - base.zeroPointOnePercentLowFps).toFixed(1);
const pointOneLowPct = (((sync.zeroPointOnePercentLowFps - base.zeroPointOnePercentLowFps) / base.zeroPointOnePercentLowFps) * 100).toFixed(1);

const ftDiff = (sync.avgFrametimeMs - base.avgFrametimeMs).toFixed(2);
const ftPct = (((sync.avgFrametimeMs - base.avgFrametimeMs) / base.avgFrametimeMs) * 100).toFixed(1);

const jitterDiff = (sync.frametimeStdDevMs - base.frametimeStdDevMs).toFixed(3);
const jitterPct = (((sync.frametimeStdDevMs - base.frametimeStdDevMs) / base.frametimeStdDevMs) * 100).toFixed(1);

console.log("==================================================================================");
console.log("       SYNCHRO HARDWARE BENCHMARK COMPARISON REPORT (RTX 3060 D3D11)");
console.log("==================================================================================");
console.log(`Metric                   | Baseline (Stock)     | With Synchro         | Delta / Impact`);
console.log("-------------------------+----------------------+----------------------+------------------");
console.log(`Average FPS              | ${String(base.avgFps).padEnd(20)} | ${String(sync.avgFps).padEnd(20)} | +${fpsDiff} FPS (+${fpsPct}%)`);
console.log(`1% Low FPS (Stutter)     | ${String(base.onePercentLowFps).padEnd(20)} | ${String(sync.onePercentLowFps).padEnd(20)} | +${oneLowDiff} FPS (+${oneLowPct}%)`);
console.log(`0.1% Low FPS (Spikes)    | ${String(base.zeroPointOnePercentLowFps).padEnd(20)} | ${String(sync.zeroPointOnePercentLowFps).padEnd(20)} | +${pointOneLowDiff} FPS (+${pointOneLowPct}%)`);
console.log(`Average Frametime        | ${String(base.avgFrametimeMs + " ms").padEnd(20)} | ${String(sync.avgFrametimeMs + " ms").padEnd(20)} | ${ftDiff} ms (${ftPct}%)`);
console.log(`Frametime Jitter (StdDev)| ${String(base.frametimeStdDevMs + " ms").padEnd(20)} | ${String(sync.frametimeStdDevMs + " ms").padEnd(20)} | ${jitterDiff} ms (${jitterPct}%)`);
console.log(`Total Frames Rendered    | ${String(base.totalFrames).padEnd(20)} | ${String(sync.totalFrames).padEnd(20)} | +${sync.totalFrames - base.totalFrames} frames`);
console.log("==================================================================================");
