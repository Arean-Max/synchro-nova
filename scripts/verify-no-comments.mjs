import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';

const repoRoot = process.cwd();
const res = execSync('git ls-files', { cwd: repoRoot, encoding: 'utf-8' });
const files = res.trim().split(/\r?\n/).filter(Boolean);

const codeExts = new Set(['.js', '.mjs', '.rs', '.css', '.html']);
const doubleSlash = String.fromCharCode(47, 47);
const slashStar = String.fromCharCode(47, 42);
const htmlComment = String.fromCharCode(60, 33, 45, 45);

const commentViolations = [];

for (const relPath of files) {
  if (relPath === 'scripts/verify-no-comments.mjs') {
    continue;
  }
  const ext = path.extname(relPath);
  if (!codeExts.has(ext)) {
    continue;
  }
  if (relPath.startsWith('src-tauri/gen/')) {
    continue;
  }

  const fullPath = path.join(repoRoot, relPath);
  let content = '';
  try {
    content = fs.readFileSync(fullPath, 'utf-8');
  } catch {
    continue;
  }

  const lines = content.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let clean = line;
    for (const protocol of ['https:', 'http:', 'asset:', 'file:']) {
      const fullProto = protocol + doubleSlash;
      clean = clean.split(fullProto).join('');
    }

    if (clean.includes(doubleSlash)) {
      commentViolations.push({ file: relPath, line: i + 1, content: line.trim() });
    }

    if (line.includes(slashStar)) {
      const stripped = line.trim();
      if (!stripped.startsWith('"test"') && !line.includes('tests/*.test.mjs')) {
        commentViolations.push({ file: relPath, line: i + 1, content: line.trim() });
      }
    }

    if (line.includes(htmlComment)) {
      commentViolations.push({ file: relPath, line: i + 1, content: line.trim() });
    }
  }
}

if (commentViolations.length > 0) {
  console.error(`Found ${commentViolations.length} comment violations:`);
  for (const v of commentViolations.slice(0, 20)) {
    console.error(`  ${v.file}:${v.line} -> ${v.content}`);
  }
  process.exit(1);
} else {
  console.log('SUCCESS: EXACTLY ZERO comments found across all source files in the repository!');
}
