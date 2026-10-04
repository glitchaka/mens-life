import { run, rustTool } from './setup.mjs';
import { existsSync, readFileSync } from 'node:fs';
import assert from 'node:assert/strict';

run(rustTool('cargo'), ['fmt','--all','--','--check']);
run(rustTool('cargo'), ['clippy','--locked','--all-targets','--','-D','warnings']);
run(rustTool('cargo'), ['clippy','--locked','--target','wasm32-unknown-unknown','--lib','--','-D','warnings']);
run(rustTool('cargo'), ['build','--locked','--example','parity']);
assert.ok(existsSync('dist/pkg/mens_life_bg.wasm'),'Run npm run build first.');
const wasm=readFileSync('dist/pkg/mens_life_bg.wasm');
assert.ok(WebAssembly.validate(wasm),'Output must be valid WebAssembly.');
run(process.execPath,['tests/parity.mjs']);
run(process.execPath,['--experimental-vm-modules','tests/wasm-ui.mjs']);
console.log('Rust checks and comparison with the original passed. GPU/browser and native Windows execution require their respective environments.');
