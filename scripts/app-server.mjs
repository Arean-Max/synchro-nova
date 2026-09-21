import { execFile } from "node:child_process";
import { readFile } from "node:fs/promises";
import { createServer } from "node:http";
import { extname, join, normalize, resolve, sep } from "node:path";
import { promisify } from "node:util";

const root = resolve("src");
const rootPrefix = root.endsWith(sep) ? root : `${root}${sep}`;
const port = Number.parseInt(process.env.PORT || "64927", 10);
const host = process.env.HOST || "127.0.0.1";
const execFileAsync = promisify(execFile);
if (!Number.isInteger(port) || port < 1 || port > 65535) {
  throw new Error("PORT must be a valid TCP port");
}
if (host !== "127.0.0.1" && host !== "localhost") {
  throw new Error("HOST is restricted to loopback for Synchro");
}

const types = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml"
};

async function stopStaleDevProcess() {
  if (process.platform !== "win32") return;
  const targetExe = resolve("src-tauri", "target", "debug", "synchro.exe").toLowerCase();
  const command = [
    "$target = $args[0].ToLowerInvariant();",
    "Get-Process synchro -ErrorAction SilentlyContinue |",
    "Where-Object { $_.Path -and $_.Path.ToLowerInvariant() -eq $target } |",
    "Stop-Process -Force"
  ].join(" ");
  await execFileAsync("powershell", [
    "-NoProfile",
    "-ExecutionPolicy",
    "Bypass",
    "-Command",
    command,
    targetExe
  ]).catch(() => {});
}

function safePath(url) {
  try {
    const parsed = new URL(url, `http://${host}:${port}`);
    const pathname = decodeURIComponent(parsed.pathname === "/" ? "/index.html" : parsed.pathname);
    const filePath = normalize(join(root, pathname));
    if (filePath !== root && !filePath.startsWith(rootPrefix)) return null;
    return filePath;
  } catch {
    return false;
  }
}

function createAppServer() {
  return createServer(async (request, response) => {
    const filePath = safePath(request.url || "/");
    if (filePath === false) {
      response.writeHead(400);
      response.end("Bad request");
      return;
    }
    if (!filePath) {
      response.writeHead(403);
      response.end("Forbidden");
      return;
    }

    try {
      const body = await readFile(filePath);
      response.writeHead(200, {
        "content-type": types[extname(filePath)] || "application/octet-stream",
        "x-content-type-options": "nosniff",
        "referrer-policy": "no-referrer"
      });
      response.end(body);
    } catch {
      response.writeHead(404);
      response.end("Not found");
    }
  });
}

async function listen() {
  const server = createAppServer();
  await new Promise((resolveListen, rejectListen) => {
    server.once("error", rejectListen);
    server.listen(port, host, resolveListen);
  });
  return server;
}

await stopStaleDevProcess();
const server = await listen();

console.log(`Synchro app server: http://${host}:${port}`);

process.on("SIGINT", () => server.close(() => process.exit(0)));
process.on("SIGTERM", () => server.close(() => process.exit(0)));
