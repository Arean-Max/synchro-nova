import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';

function scan(dir, out = []) {
  if (!fs.existsSync(dir)) return out;
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== 'node_modules' && entry.name !== '.git') {
        scan(full, out);
      }
    } else if (entry.name.endsWith('.js') || entry.name.endsWith('.mjs')) {
      out.push(full);
    }
  }
  return out;
}

const files = [...scan('src'), ...scan('scripts')];
console.log(`Verifying syntax across ${files.length} JavaScript modules...`);

let failed = 0;
for (const file of files) {
  try {
    execSync(`node --check "${file}"`, { stdio: 'pipe' });
  } catch (e) {
    console.error(`Syntax error in ${file}:\n${e.stderr?.toString() || e.message}`);
    failed++;
  }
}

if (failed > 0) {
  console.error(`Syntax verification failed for ${failed} files.`);
  process.exit(1);
} else {
  console.log(`All ${files.length} JavaScript files passed syntax verification cleanly.`);
}
