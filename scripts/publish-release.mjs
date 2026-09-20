import https from 'node:https';
import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';

let token = process.env.GITHUB_TOKEN;
if (!token) {
  try {
    const creds = execSync('git credential fill', {
      input: 'protocol=https\nhost=github.com\n',
      encoding: 'utf8'
    });
    const match = creds.match(/password=(.+)/);
    if (match) token = match[1].trim();
  } catch {}
}

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

  let release = Array.isArray(rels.data) ? rels.data.find(r => r.tag_name === 'v2.1') : null;

  const releaseBody = {
    tag_name: 'v2.1',
    target_commitish: 'main',
    name: 'Synchro Nova v2.1',
    body: [
      '## Synchro Nova v2.1',
      '',
      '### Changes & Improvements',
      '- **Security & Runtime Isolation**: Removed deprecated URLDownloadToFileW and background executable execution. Missing runtime components now trigger standard user prompts with official Microsoft download links.',
      '- **Process Isolation Architecture**: Color adjustments run via Windows Desktop Window Manager (DWM) compositor shaders and hardware display LUTs without cross-process memory access, DLL injection, or input hooks.',
      '- **Unified Settings UI**: Unified application preferences, theme toggles, and community links into a single view with standardized toggle switch bounds.',
      '- **Clean Portable Operation**: Eliminated startup registry writes. Portable mode operates self-contained without polluting user registry keys.',
      '- **Performance**: Worker synchronization via native Condvar ensures zero timer wakeups and minimal CPU/memory usage when idle.',
      '',
      '### Downloads',
      '- **Synchro-Nova-2.1.0-Portable.zip** — Standalone portable archive.',
      '- **Synchro.Nova_2.1.0_x64-setup.exe** — Windows installer package with Start Menu shortcut and clean uninstaller.'
    ].join('\n'),
    draft: false,
    prerelease: false,
    make_latest: 'true'
  };

  if (!release) {
    console.log('2. Creating Release v2.1 on GitHub...');
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
    console.log('Release v2.1 already exists with ID:', release.id, '- updating body and name...');
    const updated = await request({
      hostname: 'api.github.com',
      path: `/repos/${owner}/${repo}/releases/${release.id}`,
      method: 'PATCH',
      headers: {
        'User-Agent': 'NodeJS',
        'Authorization': 'token ' + token,
        'Accept': 'application/vnd.github.v3+json',
        'Content-Type': 'application/json'
      }
    }, JSON.stringify({
      name: releaseBody.name,
      body: releaseBody.body
    }));
    if (updated.data && updated.data.upload_url) {
      release = updated.data;
    }
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

  const setupPath = path.resolve('dist-artifacts/Synchro.Nova_2.1.0_x64-setup.exe');
  const zipPath = path.resolve('dist-artifacts/Synchro-Nova-2.1.0-Portable.zip');

  if (fs.existsSync(setupPath)) {
    console.log('4. Uploading Synchro.Nova_2.1.0_x64-setup.exe...');
    const resSetup = await uploadAsset(release.upload_url, setupPath, 'Synchro.Nova_2.1.0_x64-setup.exe', 'application/vnd.microsoft.portable-executable');
    console.log('Synchro.Nova_2.1.0_x64-setup.exe upload status:', resSetup.status);
  } else {
    console.warn('Setup file not found:', setupPath);
  }

  if (fs.existsSync(zipPath)) {
    console.log('5. Uploading Synchro-Nova-2.1.0-Portable.zip...');
    const resZip = await uploadAsset(release.upload_url, zipPath, 'Synchro-Nova-2.1.0-Portable.zip', 'application/zip');
    console.log('Synchro-Nova-2.1.0-Portable.zip upload status:', resZip.status);
  } else {
    console.warn('Portable zip file not found:', zipPath);
  }

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
