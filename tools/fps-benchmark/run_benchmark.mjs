import http from "node:http";
import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";

const ARGS = process.argv.slice(2);
const MODE = ARGS.includes("--synchro") ? "synchro" : (ARGS.includes("--baseline") ? "baseline" : "custom");
const PORT = 18930;
const BENCHMARK_DIR = path.resolve("C:\\Users\\tyler\\Desktop\\synchro\\tools\\fps-benchmark");

const mimeTypes = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".json": "application/json"
};

const server = http.createServer((req, res) => {
  let reqPath = decodeURIComponent(req.url.split("?")[0]);
  if (reqPath === "/") reqPath = "/index.html";
  const filePath = path.join(BENCHMARK_DIR, reqPath);

  if (!fs.existsSync(filePath) || fs.statSync(filePath).isDirectory()) {
    res.writeHead(404);
    res.end("Not Found");
    return;
  }

  const ext = path.extname(filePath).toLowerCase();
  const contentType = mimeTypes[ext] || "application/octet-stream";
  res.writeHead(200, { "Content-Type": contentType });
  fs.createReadStream(filePath).pipe(res);
});

server.listen(PORT, "127.0.0.1", async () => {
  console.log(`[FPS Benchmark] Server listening on http://127.0.0.1:${PORT}`);
  console.log(`[FPS Benchmark] Mode: [${MODE.toUpperCase()}]`);

  const edgePath = "C:\\Program Files (x86)\\Microsoft\\EdgeCore\\157.0.4341.46\\msedge.exe";
  const cdpPort = 9230;

  try {
    const { execSync } = await import("node:child_process");
    execSync('Stop-Process -Name msedge -Force -ErrorAction SilentlyContinue', { shell: "powershell" });
  } catch (_) {}

  await new Promise((r) => setTimeout(r, 600));

  const edgeArgs = [
    "--headless=new",
    `--remote-debugging-port=${cdpPort}`,
    "--disable-gpu-vsync",
    "--disable-frame-rate-limit",
    "--use-gl=angle",
    "--use-angle=d3d11",
    "--enable-gpu-rasterization",
    "--enable-zero-copy",
    "--window-size=1280,720",
    `http://127.0.0.1:${PORT}`
  ];

  console.log(`[FPS Benchmark] Launching hardware D3D11 rendering test...`);
  const edgeProc = spawn(edgePath, edgeArgs);

  await new Promise((r) => setTimeout(r, 2000));

  try {
    const listRes = await fetch(`http://127.0.0.1:${cdpPort}/json`);
    const targets = await listRes.json();
    const pageTarget = targets.find((t) => t.type === "page") || targets[0];
    if (!pageTarget) throw new Error("No CDP page target found");

    const ws = new WebSocket(pageTarget.webSocketDebuggerUrl);

    let msgId = 1;
    const pending = new Map();
    ws.onmessage = (event) => {
      const msg = JSON.parse(event.data.toString());
      if (msg.id && pending.has(msg.id)) {
        const resolve = pending.get(msg.id);
        pending.delete(msg.id);
        resolve(msg);
      }
    };

    const send = (method, params = {}) => {
      return new Promise((resolve) => {
        const id = msgId++;
        pending.set(id, resolve);
        ws.send(JSON.stringify({ id, method, params }));
      });
    };

    await new Promise((r) => (ws.onopen = r));
    await send("Runtime.enable");

    console.log(`[FPS Benchmark] Running real-time particle & matrix simulation (6 seconds test window)...`);

    let results = null;
    const maxAttempts = 30; // 30 seconds max
    for (let i = 0; i < maxAttempts; i++) {
      await new Promise((r) => setTimeout(r, 500));
      const evalRes = await send("Runtime.evaluate", {
        expression: "window.__BENCHMARK_RESULTS__",
        returnByValue: true
      });
      const val = evalRes?.result?.result?.value;
      if (val) {
        results = val;
        break;
      }
    }

    if (!results) {
      throw new Error("Benchmark timed out before completion.");
    }

    results.mode = MODE;
    results.gpu = "NVIDIA GeForce RTX 3060";
    results.driver = "32.0.16.2124 (D3D11/WDDM)";
    results.cpu = "12th Gen Intel Core i5-12400F";
    results.os = "Windows 10/11 x64";

    const outPath = path.join(BENCHMARK_DIR, `results_${MODE}.json`);
    fs.writeFileSync(outPath, JSON.stringify(results, null, 2), "utf8");

    console.log("\n=======================================================");
    console.log(`  SYNCHRO HARDWARE BENCHMARK: [${MODE.toUpperCase()}]`);
    console.log("=======================================================");
    console.log(`  GPU:                  ${results.gpu}`);
    console.log(`  Driver:               ${results.driver}`);
    console.log(`  Average FPS:          ${results.avgFps} FPS`);
    console.log(`  1% Low FPS:           ${results.onePercentLowFps} FPS (99th percentile)`);
    console.log(`  0.1% Low FPS:         ${results.zeroPointOnePercentLowFps} FPS (99.9th percentile)`);
    console.log(`  Average Frametime:    ${results.avgFrametimeMs} ms`);
    console.log(`  Frametime Jitter:     ${results.frametimeStdDevMs} ms (StdDev)`);
    console.log(`  Min / Max Frametime:  ${results.minFrametimeMs} ms / ${results.maxFrametimeMs} ms`);
    console.log(`  Total Frames Tested:  ${results.totalFrames} frames in ${results.durationMs} ms`);
    console.log(`  Saved to:             ${outPath}`);
    console.log("=======================================================\n");

    ws.close();
  } catch (err) {
    console.error("[FPS Benchmark Error]:", err);
  } finally {
    edgeProc.kill("SIGKILL");
    server.close();
    process.exit(0);
  }
});
