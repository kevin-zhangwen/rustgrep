#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');

const platform = os.platform();
const arch = os.arch();

const binaryName = platform === 'win32' ? 'rustgrep.exe' : 'rustgrep';

// Map platform/arch to artifact name
let artifactPlatform, artifactArch;
switch (platform) {
  case 'darwin':
    artifactPlatform = 'darwin';
    break;
  case 'linux':
    artifactPlatform = 'linux';
    break;
  default:
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
}

switch (arch) {
  case 'x64':
    artifactArch = 'x86_64';
    break;
  case 'arm64':
    artifactArch = 'aarch64';
    break;
  default:
    console.error(`Unsupported architecture: ${arch}`);
    process.exit(1);
}

const binDir = path.join(__dirname, '..', 'bin');
const binaryPath = path.join(binDir, binaryName);

async function getLatestVersion() {
  return new Promise((resolve, reject) => {
    const url = 'https://api.github.com/repos/kevin-zhangwen/rustgrep/releases/latest';
    https.get(url, {
      headers: { 'User-Agent': 'rustgrep-installer' }
    }, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          const json = JSON.parse(data);
          resolve(json.tag_name.replace('v', ''));
        } catch {
          resolve('0.1.0');
        }
      });
    }).on('error', reject);
  });
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading rustgrep from ${url}`);

    const file = fs.createWriteStream(dest);

    const request = (url) => {
      https.get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          request(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(`Download failed with status ${response.statusCode}`));
          return;
        }

        response.pipe(file);

        file.on('finish', () => {
          file.close();
          resolve();
        });
      }).on('error', (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });
    };

    request(url);
  });
}

async function install() {
  // Ensure bin directory exists
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  // Check if binary already exists
  if (fs.existsSync(binaryPath)) {
    console.log('rustgrep binary already exists, skipping download.');
    return;
  }

  const version = await getLatestVersion();
  const artifact = `rustgrep-${artifactPlatform}-${artifactArch}`;
  const downloadUrl = `https://github.com/kevin-zhangwen/rustgrep/releases/download/v${version}/${artifact}.tar.gz`;
  const tarPath = path.join(binDir, `${artifact}.tar.gz`);

  try {
    await download(downloadUrl, tarPath);

    // Extract binary
    console.log('Extracting binary...');
    execSync(`tar -xzf "${tarPath}" -C "${binDir}"`, { stdio: 'inherit' });

    // Cleanup
    fs.unlinkSync(tarPath);

    // Make executable
    fs.chmodSync(binaryPath, 0o755);

    console.log('✅ rustgrep installed successfully!');
  } catch (err) {
    console.error('Failed to download binary:', err.message);
    console.log('');
    console.log('Please build from source:');
    console.log('  git clone https://github.com/kevin-zhangwen/rustgrep.git');
    console.log('  cd rustgrep && cargo build --release');
    process.exit(1);
  }
}

install();
