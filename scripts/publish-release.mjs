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

  let release = Array.isArray(rels.data) ? rels.data.find(r => r.tag_name === 'v2.2.4') : null;

  const releaseBody = {
    tag_name: 'v2.2.4',
    target_commitish: 'main',
    name: 'Synchro Nova v2.2.4',
    body: [
      '# Synchro Nova v2.2.4',
      '',
      '[🌐 Read in English (English version)](https://github.com/Arean-Max/synchro-nova/blob/main/CHANGELOG.en.md)',
      '',
      '### Изменения и улучшения (v2.2.4)',
      '- **Индикатор видеокарты**: Аппаратный векторный значок вендора графического ускорителя (NVIDIA / AMD) в строке Black Holosight.',
      '- **Устойчивый запуск**: Устранена блокировка старта приложения одиночным мьютексом при отсутствии видимого окна на рабочем столе.',
      '- **Перезапуск от имени администратора**: Интеграция `ShellExecuteExW` с модальным родительским окном UAC, защита от самозакрытия при отмене UAC и сохранение открытой вкладки твиков.',
      '- **Изоляция профиля**: Выделен отдельный каталог пользовательских данных `EBWebView_Admin` для исключения конфликтов блокировки SQLite между уровнями прав Windows.',
      '- **Анимация тумблера**: Доработан плавный сдвиг ползунка Black Holo и мгновенный отклик интерфейса.',
      '',
      '---',
      '',
      '### Changes & Improvements (English)',
      '- **GPU Vendor Indicator**: Dynamic hardware GPU indicator (NVIDIA / AMD) displayed next to the Black Holo toggle switch.',
      '- **Reliable Startup**: Resolved single-instance mutex locks preventing application startup when no desktop window is active.',
      '- **Administrator Elevation**: Native `ShellExecuteExW` with modal UAC dialog parenting, error recovery on UAC cancellation, and automatic navigation back to the tweaks tab.',
      '- **Profile Isolation**: Separate `EBWebView_Admin` user data directory to eliminate SQLite database contention and Windows integrity level mismatch.',
      '- **Switch Animation**: Fixed sliding animation and optimistic UI responsiveness for the Black Holo switch.'
    ].join('\n'),
    draft: false,
    prerelease: false,
    make_latest: 'true'
  };

  if (!release) {
    console.log(`2. Creating Release ${releaseBody.tag_name} on GitHub...`);
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
    console.log(`Release ${releaseBody.tag_name} already exists with ID:`, release.id, '- updating body and name...');
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

  const artifacts = fs.existsSync('dist-artifacts') ? fs.readdirSync('dist-artifacts') : [];
  const setupName = artifacts.find(f => f.endsWith('-setup.exe')) || 'Synchro.Nova_2.2.4_x64-setup.exe';
  const zipName = artifacts.find(f => f.endsWith('-Portable.zip')) || 'Synchro-Nova-2.2.4-Portable.zip';

  const setupPath = path.resolve('dist-artifacts', setupName);
  const zipPath = path.resolve('dist-artifacts', zipName);
  const exePath = path.resolve('dist-artifacts/synchro.exe');
  const shaPath = path.resolve('dist-artifacts/SHA256SUMS.txt');

  if (fs.existsSync(setupPath)) {
    console.log(`4. Uploading ${setupName}...`);
    const resSetup = await uploadAsset(release.upload_url, setupPath, setupName, 'application/vnd.microsoft.portable-executable');
    console.log(`${setupName} upload status:`, resSetup.status);
  } else {
    console.warn('Setup file not found:', setupPath);
  }

  if (fs.existsSync(zipPath)) {
    console.log(`5. Uploading ${zipName}...`);
    const resZip = await uploadAsset(release.upload_url, zipPath, zipName, 'application/zip');
    console.log(`${zipName} upload status:`, resZip.status);
  } else {
    console.warn('Portable zip file not found:', zipPath);
  }

  if (fs.existsSync(exePath)) {
    console.log('6. Uploading synchro.exe...');
    const resExe = await uploadAsset(release.upload_url, exePath, 'synchro.exe', 'application/vnd.microsoft.portable-executable');
    console.log('synchro.exe upload status:', resExe.status);
  }

  if (fs.existsSync(shaPath)) {
    console.log('7. Uploading SHA256SUMS.txt...');
    const resSha = await uploadAsset(release.upload_url, shaPath, 'SHA256SUMS.txt', 'text/plain');
    console.log('SHA256SUMS.txt upload status:', resSha.status);
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
