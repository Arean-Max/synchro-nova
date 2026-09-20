import https from 'node:https';
import fs from 'node:fs';
import path from 'node:path';

const token = process.env.GITHUB_TOKEN;
const owner = 'Arean-Max';
const repo = 'synchro-nova';

function request(options, body = null) {
  return new Promise((resolve, reject) => {
    const req = https.request(options, (res) => {
      let data = '';
      res.on('data', chunk => { data += chunk; });
      res.on('end', () => {
        try {
          const json = data ? JSON.parse(data) : {};
          resolve({ status: res.statusCode, headers: res.headers, data: json, raw: data });
        } catch {
          resolve({ status: res.statusCode, headers: res.headers, raw: data });
        }
      });
    });
    req.on('error', reject);
    if (body) req.write(body);
    req.end();
  });
}

function uploadAsset(uploadUrl, filePath, fileName, contentType) {
  return new Promise((resolve, reject) => {
    const stat = fs.statSync(filePath);
    const fileStream = fs.createReadStream(filePath);
    const cleanUrl = uploadUrl.replace(/\{.*?\}$/, '') + '?name=' + encodeURIComponent(fileName);
    const urlObj = new URL(cleanUrl);

    const req = https.request({
      hostname: urlObj.hostname,
      path: urlObj.pathname + urlObj.search,
      method: 'POST',
      headers: {
        'User-Agent': 'NodeJS',
        'Authorization': 'token ' + token,
        'Content-Type': contentType,
        'Content-Length': stat.size
      }
    }, (res) => {
      let data = '';
      res.on('data', chunk => { data += chunk; });
      res.on('end', () => {
        resolve({ status: res.statusCode, raw: data });
      });
    });
    req.on('error', reject);
    fileStream.pipe(req);
  });
}

async function run() {
  console.log('1. Checking existing releases...');
  const rels = await request({
    hostname: 'api.github.com',
    path: `/repos/${owner}/${repo}/releases`,
    method: 'GET',
    headers: {
      'User-Agent': 'NodeJS',
      'Authorization': 'token ' + token,
      'Accept': 'application/vnd.github.v3+json'
    }
  });

  let release = Array.isArray(rels.data) ? rels.data.find(r => r.tag_name === 'v2.0') : null;

  if (!release) {
    console.log('2. Creating Release v2.0 on GitHub...');
    const releaseBody = {
      tag_name: 'v2.0',
      target_commitish: 'main',
      name: 'Synchro Nova v2.0 - Production Architecture & Optimization',
      body: [
        '## 🚀 Synchro Nova v2.0',
        '',
        '### ⚡ Performance & Production Architecture',
        '- **Ultra-low idle memory footprint** (~1.5 MB RAM).',
        '- **Optimized Chromium/WebView2 lifecycle**: speculative background networking, unnecessary telemetry, and GPU caching stripped for pure desktop reactivity.',
        '- **100% Anti-Cheat compliant**: Vanguard, EAC, BattlEye, Ricochet safe with zero DLL injection, process hooks, or thread tampering.',
        '- **WebView2Loader Auto-Patch**: permanently resolves the infamous Windows Bad Image `0xc000012f` error.',
        '',
        '### 🎨 UI, Community & Localization',
        '- **Integrated Community Hub**: Dedicated GitHub and Telegram (`t.me/synchronova`) cards designed seamlessly with the dark mode aesthetic.',
        '- **Zero-Scrollbar Settings Layout**: Redesigned compact proportions allowing all settings and community links to fit cleanly on one single screen.',
        '- **Zero Language Leakage**: Complete i18n locale audit ensuring 100% pure Russian or English across all menus, badges, dialogs, and impact banners.',
        '',
        '### 📦 Downloads & Verification',
        '- **synchro.exe** (Standalone portable x64 executable)',
        '- **SHA256SUMS.txt** (Official cryptographic verification hashes)'
      ].join('\n'),
      draft: false,
      prerelease: false,
      make_latest: 'true'
    };

    const created = await request({
      hostname: 'api.github.com',
      path: `/repos/${owner}/${repo}/releases`,
      method: 'POST',
      headers: {
        'User-Agent': 'NodeJS',
        'Authorization': 'token ' + token,
        'Accept': 'application/vnd.github.v3+json',
        'Content-Type': 'application/json'
      }
    }, JSON.stringify(releaseBody));

    console.log('Release creation status:', created.status);
    release = created.data;
  } else {
    console.log('Release v2.0 already exists with ID:', release.id);
  }

  if (!release || !release.upload_url) {
    console.error('Failed to get release upload_url:', release);
    return;
  }

  console.log('3. Checking and clearing existing assets if needed...');
  if (Array.isArray(release.assets)) {
    for (const asset of release.assets) {
      console.log('Deleting previous asset:', asset.name);
      await request({
        hostname: 'api.github.com',
        path: `/repos/${owner}/${repo}/releases/assets/${asset.id}`,
        method: 'DELETE',
        headers: {
          'User-Agent': 'NodeJS',
          'Authorization': 'token ' + token
        }
      });
    }
  }

  const exePath = path.resolve('dist-artifacts/synchro.exe');
  const sumPath = path.resolve('dist-artifacts/SHA256SUMS.txt');

  console.log('4. Uploading synchro.exe (~6.5MB)...');
  const res1 = await uploadAsset(release.upload_url, exePath, 'synchro.exe', 'application/vnd.microsoft.portable-executable');
  console.log('synchro.exe upload status:', res1.status);

  console.log('5. Uploading SHA256SUMS.txt...');
  const res2 = await uploadAsset(release.upload_url, sumPath, 'SHA256SUMS.txt', 'text/plain');
  console.log('SHA256SUMS.txt upload status:', res2.status);

  console.log('6. Verifying published release...');
  const finalRel = await request({
    hostname: 'api.github.com',
    path: `/repos/${owner}/${repo}/releases/${release.id}`,
    method: 'GET',
    headers: {
      'User-Agent': 'NodeJS',
      'Authorization': 'token ' + token,
      'Accept': 'application/vnd.github.v3+json'
    }
  });

  console.log('Final Release Published Successfully!');
  console.log('URL:', finalRel.data.html_url);
  console.log('Tag:', finalRel.data.tag_name);
  console.log('Assets:', (finalRel.data.assets || []).map(a => ({
    name: a.name,
    size: (a.size / 1024 / 1024).toFixed(2) + ' MB',
    download_url: a.browser_download_url
  })));
}

run().catch(console.error);
