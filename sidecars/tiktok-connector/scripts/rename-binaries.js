const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '../../../src-tauri/bin');

// Map pkg output names to Tauri target names
const mapping = {
  'tiktok-connector-macos': [
    'tiktok-connector-x86_64-apple-darwin',
    'tiktok-connector-aarch64-apple-darwin'
  ],
  'tiktok-connector-win.exe': [
    'tiktok-connector-x86_64-pc-windows-msvc.exe'
  ],
  'tiktok-connector-linux': [
    'tiktok-connector-x86_64-unknown-linux-gnu'
  ]
};

for (const [source, targets] of Object.entries(mapping)) {
  const sourcePath = path.join(binDir, source);
  if (fs.existsSync(sourcePath)) {
    for (const target of targets) {
      const targetPath = path.join(binDir, target);
      // Copy instead of rename to ensure all targets exist
      fs.copyFileSync(sourcePath, targetPath);
      console.log(`Copied ${source} to ${target}`);
    }
  } else {
    console.warn(`Source file not found: ${sourcePath}`);
  }
}
