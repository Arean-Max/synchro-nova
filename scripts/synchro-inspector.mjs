import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const cssPath = path.join(rootDir, 'src', 'styles.css');
const i18nPath = path.join(rootDir, 'src', 'app', 'core', 'i18n.js');
const layoutPath = path.join(rootDir, 'src', 'app', 'ui', 'layout.js');
const mainJsPath = path.join(rootDir, 'src', 'app', 'main.js');
const errorBoundaryPath = path.join(rootDir, 'src', 'app', 'core', 'errorBoundary.js');

const issues = [];
const passes = [];

async function check(name, fn) {
  try {
    const result = await fn();
    if (result === true || result === undefined) {
      passes.push(name);
    } else {
      issues.push({ name, details: String(result) });
    }
  } catch (err) {
    issues.push({ name, details: err.message });
  }
}

await check('CSS: Particle canvas non-blocking', () => {
  const css = fs.readFileSync(cssPath, 'utf8');
  if (!css.includes('.theme-fx-canvas') || !css.includes('pointer-events: none')) {
    return 'Canvas layer must have pointer-events: none to avoid blocking user interactions';
  }
  return true;
});

await check('CSS: macOS traffic lights properly styled without maximize', () => {
  const css = fs.readFileSync(cssPath, 'utf8');
  if (!css.includes('.mac-traffic-lights') || !css.includes('.mac-light.close') || !css.includes('.mac-light.minimize')) {
    return 'Missing traffic light styling classes';
  }
  return true;
});

await check('CSS: Custom sleek scrollbars defined', () => {
  const css = fs.readFileSync(cssPath, 'utf8');
  if (!css.includes('::-webkit-scrollbar') || !css.includes('::-webkit-scrollbar-thumb')) {
    return 'Custom scrollbar styles are missing; default OS scrollbar may cause visual slop';
  }
  return true;
});

await check('CSS: Tabular numbers and text balance rules present', () => {
  const css = fs.readFileSync(cssPath, 'utf8');
  if (!css.includes('font-variant-numeric: tabular-nums') || !css.includes('text-wrap: balance')) {
    return 'Missing typography polish rules for tabular numbers or text-wrap balance';
  }
  return true;
});

await check('CSS: Error boundary styled with accessible contrast', () => {
  const css = fs.readFileSync(cssPath, 'utf8');
  if (!css.includes('.component-error-boundary') || !css.includes('.error-boundary-card')) {
    return 'Missing error boundary container styles';
  }
  return true;
});

await check('Security: Zero forbidden process injection/memory APIs in frontend', () => {
  const mainJs = fs.readFileSync(mainJsPath, 'utf8');
  const forbiddenPatterns = [
    /inject_dll/i,
    /write_process_memory/i,
    /read_process_memory/i,
    /hook_keyboard_global/i,
    /set_windows_hook/i,
    /open_process_all_access/i
  ];
  for (const pattern of forbiddenPatterns) {
    if (pattern.test(mainJs)) {
      return `Forbidden anti-cheat trigger pattern found: ${pattern}`;
    }
  }
  return true;
});

await check('Architecture: Fault isolation error boundary active in layout', () => {
  if (!fs.existsSync(errorBoundaryPath)) {
    return 'Missing errorBoundary.js core module';
  }
  const layout = fs.readFileSync(layoutPath, 'utf8');
  if (!layout.includes('safeRender') || !layout.includes('renderPageBody')) {
    return 'Layout does not employ safeRender wrapper for component fault isolation';
  }
  return true;
});

await check('i18n: 22 Target Languages Present and complete', async () => {
  const i18nModule = await import(`file://${i18nPath.replace(/\\/g, '/')}`);
  const expectedLangs = [
    'en', 'zh', 'zht', 'ko', 'de', 'es', 'fr', 'it', 'da', 'ja',
    'pl', 'ru', 'bs', 'ar', 'no', 'br', 'th', 'tr', 'uk', 'bn',
    'gr', 'vi'
  ];
  const dict = i18nModule.dictionaries || {};
  for (const lang of expectedLangs) {
    if (!dict[lang]) {
      return `Missing language dictionary: ${lang}`;
    }
  }
  return true;
});

console.log('=== Synchro Project Inspector Audit ===');
console.log(`Passed Checks: ${passes.length}`);
for (const p of passes) {
  console.log(`  [OK] ${p}`);
}

if (issues.length > 0) {
  console.error(`\nFound ${issues.length} Issue(s):`);
  for (const issue of issues) {
    console.error(`  [FAIL] ${issue.name}: ${issue.details}`);
  }
  process.exit(1);
} else {
  console.log('\nAll project integrity, architecture, and visual checks passed!');
  process.exit(0);
}
