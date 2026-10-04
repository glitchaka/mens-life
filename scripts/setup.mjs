import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { pathToFileURL } from 'node:url';

export function rustTool(name) {
  const exe = process.platform === 'win32' ? `${name}.exe` : name;
  const dir = process.env.CARGO_HOME || join(homedir(), '.cargo');
  const path = join(dir, 'bin', exe);
  return existsSync(path) ? path : name;
}
export function run(exe, args, options = {}) {
  const r = spawnSync(exe, args, { stdio: 'inherit', ...options });
  if (r.error) throw r.error;
  if (r.status !== 0) throw new Error(`${exe} failed with exit code ${r.status}`);
}
if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  run(rustTool('rustup'), ['toolchain', 'install', '1.90.0', '--profile', 'minimal', '--component', 'rustfmt,clippy']);
  run(rustTool('rustup'), ['target', 'add', '--toolchain', '1.90.0', 'wasm32-unknown-unknown']);
  const installed = spawnSync(rustTool('wasm-bindgen'), ['--version'], { encoding: 'utf8' });
  if (!installed.stdout?.includes('0.2.104')) run(rustTool('cargo'), ['install', 'wasm-bindgen-cli', '--version', '0.2.104', '--locked', '--force']);
}
