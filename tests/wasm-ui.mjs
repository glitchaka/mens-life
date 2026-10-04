// Runs the actual compiled Wasm and its Rust DOM/event code without a browser.
// Three.js supplies real geometry and ray casting; only GPU drawing is replaced.
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { parseHTML } from 'linkedom';
import * as THREE from 'three';
import { make_scene, bounds_for, update_world } from '../dist/bridge.js';

const {window,document}=parseHTML('<html><body><div id="app"></div></body></html>');
let scheduled,now=0,world,builds=0;
class Window {
  static [Symbol.hasInstance](value) { return value?.document===document; }
}
class PointerEvent extends window.Event {
  constructor(type,options={}) { super(type,{bubbles:true,...options});this.clientX=options.clientX||0;this.clientY=options.clientY||0; }
}
const frame=callback=>{scheduled=callback;return 1;};
const clock={now:()=>now};
const context=vm.createContext({
  console,TextEncoder,TextDecoder,Uint8Array,Int32Array,DataView,WebAssembly,URL,Response,Request,
  window,self:window,document,Window,HTMLElement:window.HTMLElement,HTMLInputElement:window.HTMLInputElement,
  HTMLSelectElement:window.HTMLSelectElement,PointerEvent,performance:clock,requestAnimationFrame:frame,
  WASM_BYTES:readFileSync('dist/pkg/mens_life_bg.wasm'),
});
// Web-sys obtains globalThis; expose the same document/clock on both surfaces.
window.requestAnimationFrame=frame;
let picked=[];
const adapter={
  create_renderer(host,json) {
    const spec=JSON.parse(json);
    host.innerHTML='';host.appendChild(document.createElement('canvas'));
    world=make_scene(spec);
    const a=1280/800,v=Math.max(spec.camera.resize.min_vertical,spec.camera.resize.min_horizontal/a);
    Object.assign(world.camera,{left:-v*a,right:v*a,top:v,bottom:-v});world.camera.updateProjectionMatrix();
    builds++;
    return JSON.stringify(bounds_for(world,spec.obstacles));
  },
  apply_updates(json){update_world(world,JSON.parse(json));},
  render_frame(){world.scene.updateMatrixWorld();world.camera.updateMatrixWorld();},
  pick_scene(x,y){
    const ray=new THREE.Raycaster();ray.setFromCamera(new THREE.Vector2(x/1280*2-1,-y/800*2+1),world.camera);
    const names=ray.intersectObjects(world.scene.children,true).map(h=>h.object.name);
    const hit=ray.intersectObject(world.nodes.get('floor'))[0];
    picked=names;
    return JSON.stringify({names,floor:hit?{x:hit.point.x,y:hit.point.y,z:hit.point.z}:null});
  },
};
const bridge=new vm.SyntheticModule(Object.keys(adapter),function(){for(const [key,value] of Object.entries(adapter))this.setExport(key,value);},{context});
const bindings=new vm.SourceTextModule(readFileSync('dist/pkg/mens_life.js','utf8'),{context,identifier:'file:///pkg/mens_life.js'});
await bindings.link(specifier=>{assert.equal(specifier,'../bridge.js');return bridge;});
const entry=new vm.SourceTextModule("import {initSync} from './mens_life.js';initSync({module:WASM_BYTES});",{context});
await entry.link(()=>bindings);await entry.evaluate();
assert.equal(builds,1);
assert.ok(document.querySelector('#scene canvas'));
assert.equal(document.querySelectorAll('.need').length,4);
assert.equal(document.querySelector('#house-name').textContent,'CASA DE ALEX');
function click(id){document.getElementById(id).dispatchEvent(new window.Event('click',{bubbles:true}));}
function tick(count=1){for(let i=0;i<count;i++){now+=50;assert.ok(scheduled);scheduled(now);}}
function input(id,value){const e=document.getElementById(id);e.value=value;e.dispatchEvent(new window.Event(id==='body'?'change':'input',{bubbles:true}));}
tick(10);
assert.equal(document.querySelector('#clock').textContent,'09:42','Creator must stop simulation');
click('play');assert.ok(document.getElementById('creator').hasAttribute('hidden'));
tick(10);assert.equal(document.querySelectorAll('.need').length,5);
for(const [id,going,using] of [
  ['bed','Yendo a la cama','Durmiendo'],['tv','Yendo a ver televisión','Viendo televisión'],
  ['shower','Yendo a la ducha','Duchándose'],['toilet','Yendo a el baño','Usando el baño'],['eat','Yendo a comer','Comiendo'],
]) {
  click(id);assert.equal(document.querySelector('#action').textContent,going);
  let reached=false;
  for(let i=0;i<550;i++){tick();if(document.querySelector('#action').textContent===using){reached=true;break;}}
  assert.ok(reached,`Actual Wasm must reach ${id}`);
  tick(25);
  if(id==='bed')assert.equal(world.nodes.get('plumb').visible,false);
  if(id==='shower'){assert.equal(world.nodes.get('drops').visible,true);const y=world.nodes.get('drop-0').position.y;tick(2);assert.notEqual(world.nodes.get('drop-0').position.y,y);}
  if(id==='tv')assert.equal(world.nodes.get('tv-screen').material.color.getHex(),0x79a7d3);
}
click('pause');const position=world.nodes.get('person').position.toArray();const clockText=document.querySelector('#clock').textContent;
tick(100);assert.deepEqual(world.nodes.get('person').position.toArray(),position);assert.equal(document.querySelector('#clock').textContent,clockText);assert.equal(document.querySelector('#pause').textContent,'▶');
click('pause');assert.equal(document.querySelector('#pause').textContent,'Ⅱ');
// Click the bed using real Three.js ray casting rather than a synthetic task ID.
tick();const projected=new THREE.Vector3(-4.25,1.3,3.3).project(world.camera);
document.querySelector('#scene canvas').dispatchEvent(new PointerEvent('pointerdown',{clientX:Math.round((projected.x+1)/2*1280),clientY:Math.round((1-projected.y)/2*800)}));
assert.ok(picked.includes('bed'));assert.equal(document.querySelector('#action').textContent,'Yendo a la cama');
click('menu');assert.equal(document.getElementById('creator').hasAttribute('hidden'),false);
input('name','Ñora <script>');assert.equal(document.querySelector('#house-name').textContent,'CASA DE ÑORA <SCRIPT>');assert.equal(document.querySelector('#avatar-letter').textContent,'Ñ');assert.equal(document.querySelector('#avatar-name').textContent,'Ñora <script>');assert.equal(document.querySelectorAll('script').length,0);
input('name','');assert.equal(document.getElementById('name').value,'Alex');
input('skin','#ffccaa');assert.equal(world.nodes.get('head').material.color.getHexString(),'ffccaa');
input('hair','#cbd5e1');assert.equal(world.nodes.get('hair').material.color.getHexString(),'cbd5e1');
input('shirt','#ff0000');assert.equal(world.nodes.get('torso').material.color.getHexString(),'ff0000');
input('pants','#00ff00');assert.equal(world.nodes.get('hips').material.color.getHexString(),'00ff00');
// Linkedom does not implement a browser's mutable select.value property.
const select=document.getElementById('body');
Object.defineProperty(select,'value',{value:'atlético',configurable:true});
select.dispatchEvent(new window.Event('change',{bubbles:true}));
assert.equal(world.nodes.get('torso').geometry.parameters.radius,0.44);
assert.equal(document.querySelectorAll('#scene canvas').length,1,'Avatar edits must replace the previous canvas');
click('play');tick(30);assert.equal(document.getElementById('creator').hasAttribute('hidden'),true);
console.log(`Compiled Wasm UI: all five actions, pause/resume, scene click, creator, Unicode, name escaping and colors passed (${builds} scene rebuilds). GPU drawing was not exercised.`);
