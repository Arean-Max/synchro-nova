#!/usr/bin/env node
/**
 * MCP (Model Context Protocol) Server: UI Layout & Irregularity Inspector
 * 
 * Provides automated inspection of frontend HTML, CSS, and JS components
 * to detect layout irregularities, clipping bugs, selector mismatches,
 * scrollbar overflow issues, and alignment flaws.
 */

import fs from 'fs';
import path from 'path';
import readline from 'readline';

const PROJECT_ROOT = process.env.SYNCHRO_ROOT || 'C:\\Users\\MARCELINE\\Desktop\\synchro';

// Core layout rules and pattern checkers
function inspectCssRules(cssContent) {
  const issues = [];
  const lines = cssContent.split('\n');

  lines.forEach((line, idx) => {
    const lineNum = idx + 1;

    // 1. Check for div:first-child or element:first-child that could fail if dynamic elements (img/icon) exist
    if (/(?:div|span|p|section):first-child/i.test(line)) {
      issues.push({
        type: 'FRAGILE_SELECTOR',
        severity: 'HIGH',
        file: 'src/styles.css',
        line: lineNum,
        snippet: line.trim(),
        message: '":first-child" selector is fragile when dynamic elements (like <img> or watermarks) are prepended. Use explicit semantic classes.'
      });
    }

    // 2. Text-overflow protections: check if white-space: nowrap is used on text blocks without ellipsis
    if (/white-space:\s*nowrap/i.test(line)) {
      const contextLines = lines.slice(Math.max(0, idx - 6), Math.min(lines.length, idx + 6));
      const context = contextLines.join(' ');
      const isButtonOrTag = /(?:\.btn|\.tag|\.badge|\.seg-option|\.seg-tab|button\b)/i.test(context);
      
      if (!isButtonOrTag && !/overflow:\s*hidden/i.test(context) && !/text-overflow:\s*ellipsis/i.test(context)) {
        issues.push({
          type: 'OVERFLOW_RISK',
          severity: 'MEDIUM',
          file: 'src/styles.css',
          line: lineNum,
          snippet: line.trim(),
          message: '"white-space: nowrap" on content container without "overflow: hidden" and "text-overflow: ellipsis" risks clipping text.'
        });
      }
    }
  });

  // 3. Global Scrollbar Check
  const hasGlobalWebkitScrollbar = /^\s*::-webkit-scrollbar\b/m.test(cssContent);
  if (!hasGlobalWebkitScrollbar) {
    issues.push({
      type: 'MISSING_SCROLLBAR_THEME',
      severity: 'HIGH',
      file: 'src/styles.css',
      line: 1,
      snippet: '::-webkit-scrollbar missing',
      message: 'Global ::-webkit-scrollbar rule is missing. WebView2 will render native 17px white Windows scrollbars that overlap dark cards.'
    });
  }

  // 4. Check for scrollbar-gutter on scrollable drawer/panels
  if (/color-game-panel\s*\{/i.test(cssContent)) {
    const match = cssContent.match(/\.color-game-panel\s*\{([^}]+)\}/i);
    if (match && !/scrollbar-gutter\s*:\s*stable/i.test(match[1])) {
      issues.push({
        type: 'SCROLLBAR_CONTENT_CLIPPING',
        severity: 'HIGH',
        file: 'src/styles.css',
        snippet: '.color-game-panel',
        message: '.color-game-panel lacks "scrollbar-gutter: stable", causing scrollbar to clip right-aligned action buttons.'
      });
    }
  }

  // 5. Check for color-scheme: dark on :root / html
  if (!/color-scheme\s*:\s*dark/i.test(cssContent)) {
    issues.push({
      type: 'MISSING_DARK_COLOR_SCHEME',
      severity: 'HIGH',
      file: 'src/styles.css',
      line: 1,
      snippet: ':root missing color-scheme: dark',
      message: ':root and html must declare "color-scheme: dark" to prevent WebView2 native Windows Fluent scrollbars and controls from defaulting to light theme.'
    });
  }

  // 6. Check for card image overlay suppression
  if (!/\.game-card\[data-has-image="true"\]\s*\.game-card-logo/i.test(cssContent)) {
    issues.push({
      type: 'CARD_TEXT_OVERLAY_ON_COVER',
      severity: 'MEDIUM',
      file: 'src/styles.css',
      line: 1,
      snippet: '.game-card[data-has-image="true"] .game-card-logo',
      message: 'When game cards have cover artwork, text logos must be suppressed to avoid obscuring poster art.'
    });
  }

  return issues;
}

function inspectJsComponent(filePath, content) {
  const issues = [];
  const lines = content.split('\n');

  lines.forEach((line, idx) => {
    const lineNum = idx + 1;

    // Check for inline span + strong collisions without dedicated wrappers or classes
    if (/<span>\$\{t\("selectedGame"\)\}<\/span>/i.test(line) && /<div>/i.test(line)) {
      issues.push({
        type: 'INLINE_COLLISION',
        severity: 'HIGH',
        file: filePath,
        line: lineNum,
        snippet: line.trim(),
        message: 'Selected game span and title inside unclassed <div> render inline if CSS fails, causing text to collide.'
      });
    }

    // Check for unclassed generic actions in narrow headers
    if (/<div class="actions">\s*\$\{launch\}/i.test(line)) {
      issues.push({
        type: 'ACTION_CLIPPING_RISK',
        severity: 'HIGH',
        file: filePath,
        line: lineNum,
        snippet: line.trim(),
        message: 'Launch action button inside generic .actions risks being compressed or clipped if drawer width shrinks.'
      });
    }

    // Check for word truncation like slice(0, 10) in logo text
    if (/word\.slice\(0,\s*\d+\)/i.test(line)) {
      issues.push({
        type: 'WORD_TRUNCATION_ARTIFACT',
        severity: 'HIGH',
        file: filePath,
        line: lineNum,
        snippet: line.trim(),
        message: 'Blind character slicing on game title words cuts words prematurely (e.g. Counter-Strike -> COUNTER-ST).'
      });
    }
  });

  return issues;
}

export function runFullInspection(focus = null) {
  const results = {
    timestamp: new Date().toISOString(),
    focus: focus || 'all',
    scannedFiles: [],
    issues: [],
    summary: {}
  };

  const cssPath = path.join(PROJECT_ROOT, 'src', 'styles.css');
  if (fs.existsSync(cssPath)) {
    results.scannedFiles.push('src/styles.css');
    const cssContent = fs.readFileSync(cssPath, 'utf8');
    results.issues.push(...inspectCssRules(cssContent));
  }

  const jsFiles = [
    'src/app/features/color/page.js',
    'src/app/features/tweaks/page.js',
    'src/app/features/settings/page.js',
    'src/app/features/monitor/page.js',
    'src/app/features/storage/page.js',
    'src/app/main.js'
  ];

  for (const rel of jsFiles) {
    if (focus && !rel.includes(focus)) continue;
    const fullPath = path.join(PROJECT_ROOT, rel);
    if (fs.existsSync(fullPath)) {
      results.scannedFiles.push(rel);
      const content = fs.readFileSync(fullPath, 'utf8');
      results.issues.push(...inspectJsComponent(rel, content));
    }
  }

  results.summary = {
    totalIssues: results.issues.length,
    highSeverity: results.issues.filter(i => i.severity === 'HIGH').length,
    mediumSeverity: results.issues.filter(i => i.severity === 'MEDIUM').length,
    status: results.issues.length === 0 ? 'CLEAN_LAYOUT' : 'ISSUES_DETECTED'
  };

  return results;
}

// MCP JSON-RPC Stdio Server Protocol Handler
function startMcpServer() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
  });

  function sendResponse(id, result, error = null) {
    const response = {
      jsonrpc: '2.0',
      id
    };
    if (error) {
      response.error = error;
    } else {
      response.result = result;
    }
    process.stdout.write(JSON.stringify(response) + '\n');
  }

  rl.on('line', (line) => {
    if (!line.trim()) return;
    try {
      const request = JSON.parse(line);
      const { id, method, params } = request;

      if (method === 'initialize') {
        sendResponse(id, {
          protocolVersion: '2024-11-05',
          capabilities: {
            tools: {}
          },
          serverInfo: {
            name: 'ui-layout-inspector',
            version: '1.0.0'
          }
        });
      } else if (method === 'notifications/initialized') {
        // notification, no response required
      } else if (method === 'tools/list') {
        sendResponse(id, {
          tools: [
            {
              name: 'inspect_ui_layout',
              description: 'Inspects frontend CSS/HTML/JS for layout irregularities, text clipping, scrollbar collisions, and selector bugs.',
              inputSchema: {
                type: 'object',
                properties: {
                  focus: {
                    type: 'string',
                    description: 'Optional feature area to focus on, e.g. "color", "game_profile", "tweaks", "settings"'
                  }
                }
              }
            }
          ]
        });
      } else if (method === 'tools/call') {
        const { name, arguments: args } = params || {};
        if (name === 'inspect_ui_layout') {
          const report = runFullInspection(args?.focus);
          sendResponse(id, {
            content: [
              {
                type: 'text',
                text: JSON.stringify(report, null, 2)
              }
            ]
          });
        } else {
          sendResponse(id, null, { code: -32601, message: `Tool not found: ${name}` });
        }
      } else {
        if (id !== undefined) {
          sendResponse(id, null, { code: -32601, message: `Method not found: ${method}` });
        }
      }
    } catch (err) {
      // ignore parse error or invalid json
    }
  });
}

// CLI check or Stdio mode
if (process.argv.includes('--run') || process.argv.includes('--inspect')) {
  console.log(JSON.stringify(runFullInspection(), null, 2));
} else {
  startMcpServer();
}
