#!/usr/bin/env node

/**
 * VaultX Uninstall Script
 * Cleans up downloaded binaries
 */

const path = require('path');
const fs = require('fs');

const BINARY_DIR = path.join(__dirname, '..', 'binaries');

try {
  if (fs.existsSync(BINARY_DIR)) {
    // List all files in binaries directory
    const files = fs.readdirSync(BINARY_DIR);
    
    // Remove each file
    files.forEach(file => {
      const filePath = path.join(BINARY_DIR, file);
      try {
        fs.unlinkSync(filePath);
        console.log(`Removed: ${file}`);
      } catch (err) {
        console.error(`Failed to remove ${file}:`, err.message);
      }
    });

    // Try to remove the directory if empty
    try {
      fs.rmdirSync(BINARY_DIR);
      console.log('Cleaned up binaries directory');
    } catch (err) {
      // Directory might not be empty, that's OK
    }
  }
  
  console.log('✅ Cleanup complete');
} catch (error) {
  console.error('❌ Cleanup error:', error.message);
  // Don't fail the uninstall process
  process.exit(0);
}
