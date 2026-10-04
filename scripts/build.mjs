import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { rustTool, run } from './setup.mjs';
import { compile } from 'tailwindcss';
import { homedir } from 'node:os';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
process.chdir(root);
if (!existsSync('node_modules/three/package.json')) throw new Error('Run npm ci first.');
const dest = join(root, 'dist');
mkdirSync(join(dest, 'vendor'), { recursive: true });
cpSync('web', dest, { recursive: true });
for (const name of ['three.module.js', 'three.core.js']) cpSync(`node_modules/three/build/${name}`, join(dest,'vendor',name));
writeFileSync(join(dest,'vendor/RoundedBoxGeometry.js'), readFileSync('node_modules/three/examples/jsm/geometries/RoundedBoxGeometry.js','utf8').replace("from 'three'", "from './three.module.js'"));
cpSync('node_modules/three/LICENSE', join(dest,'vendor/THREE-LICENSE.txt'));
// Compile Tailwind's original reset; --theme() in its source is a build directive.
const reset = await compile(readFileSync('node_modules/tailwindcss/index.css','utf8'), {
  base: join(root,'node_modules/tailwindcss'),
  loadStylesheet: async (id, base) => {
    const path = join(base,id);
    return { path, base: dirname(path), content: readFileSync(path,'utf8') };
  },
});
writeFileSync(join(dest,'preflight.css'), reset.build(['antialiased']));
cpSync('node_modules/tailwindcss/LICENSE', join(dest,'vendor/TAILWIND-LICENSE.txt'));
run(rustTool('cargo'), ['build','--locked','--release','--target','wasm32-unknown-unknown','--lib']);
run(rustTool('wasm-bindgen'), ['--target','web','--out-dir','dist/pkg','--out-name','mens_life','target/wasm32-unknown-unknown/release/mens_life.wasm']);
if (process.argv.includes('--windows')) {
  let executable;
  if (process.platform === 'win32') {
    run(rustTool('cargo'), ['build','--locked','--release','--features','desktop','--bin','mens-life']);
    executable = 'target/release/mens-life.exe';
  } else {
    run(rustTool('rustup'), ['target','add','x86_64-pc-windows-gnu']);
    run(rustTool('cargo'), ['build','--locked','--release','--target','x86_64-pc-windows-gnu','--features','desktop','--bin','mens-life']);
    executable = 'target/x86_64-pc-windows-gnu/release/mens-life.exe';
  }
  const release = join(root,'artifacts','mens-life-windows-x64');
  mkdirSync(release,{recursive:true});
  cpSync(executable,join(release,'mens-life.exe'));
  const registry = join(process.env.CARGO_HOME || join(homedir(),'.cargo'),'registry','src');
  const loader = readdirSync(registry).map(dir=>join(registry,dir,'webview2-com-sys-0.38.2','x64','WebView2Loader.dll')).find(existsSync);
  if (!loader) throw new Error('WebView2Loader.dll was not found; desktop packaging is incomplete.');
  cpSync(loader,join(release,'WebView2Loader.dll'));
  writeFileSync(join(release,'LEEME.txt'),'Vida Isométrica 3D / Mens Life\n\nAbre mens-life.exe. Conserva WebView2Loader.dll junto al ejecutable.\nRequiere Windows 10/11 x64 y Microsoft Edge WebView2 Runtime.\nLa escena, WebAssembly y recursos están incluidos; no requiere conexión.\n');
  console.log('Windows x64 executable and loader ready in artifacts/mens-life-windows-x64/.');
}
console.log('Rust/WebAssembly build ready in dist/.');
