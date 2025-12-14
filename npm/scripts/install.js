#!/usr/bin/env node

/**
 * VaultX Installation Script
 * 
 * This script detects the user's platform and architecture,
 * then downloads the appropriate pre-built binary from GitHub releases.
 */

const os = require('os');
const path = require('path');
const fs = require('fs');
const https = require('https');
const { execSync } = require('child_process');

const GITHUB_REPO = 'skdas20/VaultX';
const RELEASE_TAG = 'latest'; // or specific version like 'v0.1.0'
const BINARY_DIR = path.join(__dirname, '..', 'binaries');

// Detect platform and architecture
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  let binaryName = null;
  let downloadUrl = null;

  switch (platform) {
    case 'linux':
      if (arch === 'x64') {
        binaryName = 'vx-linux-x64';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/vx-linux-x64`;
      } else if (arch === 'arm64') {
        console.warn('⚠️  ARM64 Linux not yet available. Please build from source.');
        process.exit(1);
      } else {
        console.error(`❌ Unsupported Linux architecture: ${arch}`);
        process.exit(1);
      }
      break;

    case 'darwin':
      if (arch === 'x64') {
        binaryName = 'vx-macos-x64';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/vx-macos-x64`;
      } else if (arch === 'arm64') {
        binaryName = 'vx-macos-arm64';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/vx-macos-arm64`;
      } else {
        console.error(`❌ Unsupported macOS architecture: ${arch}`);
        process.exit(1);
      }
      break;

    case 'win32':
      if (arch === 'x64') {
        binaryName = 'vx-windows-x64.exe';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/vx-windows-x64.exe`;
      } else {
        console.error(`❌ Unsupported Windows architecture: ${arch}`);
        process.exit(1);
      }
      break;

    default:
      console.error(`❌ Unsupported platform: ${platform}`);
      process.exit(1);
  }

  return { platform, arch, binaryName, downloadUrl };
}

// Download file from GitHub
function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    // For GitHub releases, we need to follow redirects
    const maxRedirects = 5;
    let redirectCount = 0;

    function download(downloadUrl) {
      if (redirectCount > maxRedirects) {
        reject(new Error('Too many redirects'));
        return;
      }

      console.log(`📥 Downloading from: ${downloadUrl}`);

      https.get(downloadUrl, (response) => {
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          redirectCount++;
          download(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(`Download failed: HTTP ${response.statusCode}`));
          return;
        }

        const file = fs.createWriteStream(dest);
        let downloadedBytes = 0;
        const totalBytes = parseInt(response.headers['content-length'], 10);

        response.on('data', (chunk) => {
          downloadedBytes += chunk.length;
          const percent = Math.round((downloadedBytes / totalBytes) * 100);
          process.stdout.write(`\r  Progress: ${percent}%`);
        });

        response.pipe(file);

        file.on('finish', () => {
          file.close();
          console.log('\n✅ Download complete');
          resolve();
        });

        file.on('error', (err) => {
          fs.unlink(dest, () => {}); // Delete the file on error
          reject(err);
        });
      }).on('error', reject);
    }

    download(url);
  });
}

// Main installation function
async function install() {
  try {
    console.log('🔧 VaultX Installation');
    console.log('═════════════════════════════════════════');
    console.log('');

    // Detect platform
    const { platform, arch, binaryName, downloadUrl } = getPlatform();
    console.log(`✓ Detected: ${platform} (${arch})`);
    console.log(`✓ Binary: ${binaryName}`);
    console.log('');

    // Create binaries directory
    if (!fs.existsSync(BINARY_DIR)) {
      fs.mkdirSync(BINARY_DIR, { recursive: true });
    }

    // Download binary
    console.log('📦 Downloading binary...');
    const binaryPath = path.join(BINARY_DIR, binaryName);
    await downloadFile(downloadUrl, binaryPath);

    // Make executable (Linux/macOS)
    if (platform !== 'win32') {
      console.log('🔐 Setting permissions...');
      fs.chmodSync(binaryPath, 0o755);
    }

    console.log('');
    console.log('═════════════════════════════════════════');
    console.log('✅ Installation successful!');
    console.log('');
    console.log('🚀 To get started:');
    console.log('  vx --version   # Check version');
    console.log('  vx --help      # Show all commands');
    console.log('  vx init        # Create a new vault');
    console.log('');
    console.log('📖 Learn more: https://github.com/skdas20/VaultX#quickstart');
    console.log('');
  } catch (error) {
    console.error('');
    console.error('❌ Installation failed:', error.message);
    console.error('');
    console.error('💡 Possible solutions:');
    console.error('  1. Check your internet connection');
    console.error('  2. Verify the binary exists for your platform');
    console.error('  3. Try building from source: https://github.com/skdas20/VaultX');
    process.exit(1);
  }
}

// Run installation if this script is executed directly
if (require.main === module) {
  install();
}

module.exports = { getPlatform, downloadFile };
