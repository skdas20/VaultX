#!/usr/bin/env node

/**
 * VaultX CLI Entry Point
 * 
 * This script locates and executes the appropriate binary
 * for the user's platform.
 */

const os = require('os');
const path = require('path');
const fs = require('fs');
const { execSync } = require('child_process');

function findBinary() {
  const platform = process.platform;
  const arch = process.arch;
  const binariesDir = path.join(__dirname, '..', 'binaries');

  let binaryName = null;

  // Determine binary name based on platform
  switch (platform) {
    case 'linux':
      if (arch === 'x64') binaryName = 'vx-x86_64-unknown-linux-gnu';
      break;
    case 'darwin':
      if (arch === 'x64') binaryName = 'vx-x86_64-apple-darwin';
      else if (arch === 'arm64') binaryName = 'vx-aarch64-apple-darwin';
      break;
    case 'win32':
      if (arch === 'x64') binaryName = 'vx-x86_64-pc-windows-msvc.exe';
      break;
  }

  if (!binaryName) {
    console.error(`❌ Unsupported platform: ${platform} (${arch})`);
    process.exit(1);
  }

  const binaryPath = path.join(binariesDir, binaryName);

  if (!fs.existsSync(binaryPath)) {
    console.error(`❌ Binary not found: ${binaryPath}`);
    console.error('');
    console.error('This might happen if:');
    console.error('  1. Installation was interrupted');
    console.error('  2. The binaries directory was deleted');
    console.error('');
    console.error('Try reinstalling:');
    console.error('  npm install -g vaultx');
    process.exit(1);
  }

  return binaryPath;
}

try {
  const binaryPath = findBinary();
  
  // Get all arguments passed to this script (skip node and script name)
  const args = process.argv.slice(2);

  // Execute the binary with all arguments
  // Use execFileSync instead of execSync to properly handle binary execution
  const { execFileSync } = require('child_process');
  
  try {
    execFileSync(binaryPath, args, {
      stdio: 'inherit',
      windowsHide: false
    });
  } catch (error) {
    // The binary executed but returned non-zero exit code
    // Just exit with the same code
    process.exit(error.status || 1);
  }
} catch (error) {
  console.error('❌ Error:', error.message);
  process.exit(1);
}
