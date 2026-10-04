import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import vm from 'node:vm';
import ts from 'typescript';
import * as THREE from 'three';
import { RoundedBoxGeometry } from 'three/addons/geometries/RoundedBoxGeometry.js';
import * as React from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { parseHTML } from 'linkedom';
import { make_scene, bounds_for } from '../dist/bridge.js';

const original = readFileSync(new URL('./reference/original-page.tsx', import.meta.url),'utf8');
const executable = process.platform === 'win32' ? 'target/debug/examples/parity.exe' : 'target/debug/examples/parity';
function rust(input) {
  const result = spawnSync(executable, [], { input: JSON.stringify(input), encoding:'utf8', maxBuffer:100*1024*1024 });
  assert.equal(result.status,0,result.stderr);
  return JSON.parse(result.stdout);
}
function close(actual, expected, path = '', tolerance = 1e-10) {
  if (typeof expected === 'number') {
    assert.ok(typeof actual === 'number' && Math.abs(actual-expected) <= tolerance,
      `${path}: Rust ${actual}, original ${expected}`);
    return;
  }
  if (Array.isArray(expected)) {
    assert.ok(Array.isArray(actual),path);
    assert.equal(actual.length,expected.length,`${path}.length`);
    expected.forEach((x,i)=>close(actual[i],x,`${path}[${i}]`,tolerance));
    return;
  }
  if (expected && typeof expected === 'object') {
    assert.deepEqual(Object.keys(actual).sort(),Object.keys(expected).sort(),`${path}.keys`);
    for(const key of Object.keys(expected)) close(actual[key],expected[key],`${path}.${key}`,tolerance);
    return;
  }
  assert.equal(actual,expected,path);
}
const defaultAvatar = {name:'Alex',body:'adulto',skin:'#b97855',hair:'#37271f',shirt:'#315f79',pants:'#27384b'};
let seed=12345678;
const random = Array.from({length:90},()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/2**32;});
function reference(avatar = defaultAvatar) {
  let refIndex=0,stateIndex=0,randomIndex=0;
  const ui={needs:[],action:'Nada pendiente',clock:'09:42'};
  let scheduled;
  const host={clientWidth:1280,clientHeight:800,appendChild(){},removeChild(){}};
  class Renderer {
    constructor(){this.domElement={addEventListener(){},removeEventListener(){},getBoundingClientRect:()=>({left:0,top:0,width:1280,height:800})};this.shadowMap={};}
    setPixelRatio(){} setSize(){} dispose(){}
    render(scene,camera){scene.updateMatrixWorld();camera.updateMatrixWorld();}
  }
  const context=vm.createContext({
    THREE:{...THREE,WebGLRenderer:Renderer},RoundedBoxGeometry,
    useRef:initial=>({current:refIndex++===0?host:initial}),
    useState:initial=>{const i=stateIndex++;const value=i===5?avatar:initial;if(i===0)ui.needs=value;return [value,newValue=>{if(i===0)ui.needs=newValue;if(i===1)ui.action=newValue;if(i===2)ui.clock=newValue;}];},
    useEffect:fn=>fn(),devicePixelRatio:1,performance:{now:()=>0},
    ResizeObserver:class{observe(){}disconnect(){}},
    requestAnimationFrame:fn=>{scheduled=fn;return 1;},cancelAnimationFrame(){},
    Math:Object.assign(Object.create(Math),{random:()=>random[randomIndex++]}),
    UI:ui,
  });
  let source=original.replace(/^import .*$/gm,'').replace('export default function Home','function Home');
  source=source.slice(0,source.indexOf(' return <main'))+'\n}\nHome();';
  const capture=`
 globalThis.REF = { scene,camera,person,legs,arms,plumb,drops,screenMat,free,bounds,control,command,blocked,route,
 setValues:v=>{values=v},
 walk:point=>{ if(control.current.creator||control.current.paused||blocked(point.x,point.z))return;restore();const next=route(person.position,point);if(next){path=next;task='';phase='walking';setAction('Caminando')} },
 snapshot:()=>({values:[...values],path:path.map(p=>({x:p.x,y:p.y,z:p.z})),task,phase,elapsed,decisionIn,uiTime,gameTime,walkTime,action:UI.action,position:{x:person.position.x,y:person.position.y,z:person.position.z},rotation:[person.rotation.x,person.rotation.y,person.rotation.z],legs:legs.map(l=>l.rotation.x),arms:arms.map(l=>l.rotation.x),plumb:plumb.visible,drops:drops.visible,dropsY:drops.children.map(d=>d.position.y),watching:screenMat.color.getHex()===0x79a7d3,screenHue:LAST_HUE,displayedValues:UI.needs.length===4?[...UI.needs.map(n=>n.value),70]:UI.needs.map(n=>n.value),displayedClock:UI.clock,needsVisible:UI.needs.length})
 };`;
  source=source.replace('screenMat.emissive.setHSL((now/14000)%1', 'LAST_HUE=(now/14000)%1;screenMat.emissive.setHSL((now/14000)%1');
  source=source.replace(' return()=>{',capture+'\n return()=>{');
  const compiled=ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ES2022,jsx:ts.JsxEmit.React}}).outputText;
  context.LAST_NOW=0; context.LAST_HUE=0;
  vm.runInContext(compiled,context);
  return {ref:context.REF,context,tick:now=>{context.LAST_NOW=now;scheduled(now);},ui};
}
function describe(scene) {
  const result=[];
  scene.traverse(o=>{
    if(o===scene)return;
    const s={type:o.type,name:o.name,position:o.position.toArray(),rotation:[o.rotation.x,o.rotation.y,o.rotation.z],scale:o.scale.toArray(),visible:o.visible};
    if(o.isMesh){
      s.castShadow=o.castShadow;s.receiveShadow=o.receiveShadow;
      s.material={type:o.material.type,color:o.material.color.getHex(),roughness:o.material.roughness??null,metalness:o.material.metalness??null,opacity:o.material.opacity,transparent:o.material.transparent,emissive:o.material.emissive?.getHex()??null};
      s.geometry={type:o.geometry.type,parameters:o.geometry.parameters,attributes:Object.fromEntries(Object.entries(o.geometry.attributes).map(([key,a])=>[key,{count:a.count,itemSize:a.itemSize,hash:createHash('sha256').update(Buffer.from(a.array.buffer,a.array.byteOffset,a.array.byteLength)).digest('hex')}]))};
    }
    if(o.isLight){s.color=o.color.getHex();s.intensity=o.intensity;if(o.groundColor)s.groundColor=o.groundColor.getHex();if(o.distance!==undefined)s.distance=o.distance;if(o.shadow){s.castShadow=o.castShadow;s.shadowSize=o.shadow.mapSize.toArray();}}
    result.push(s);
  });
  return result;
}
function originalUI(avatar) {
  let stateIndex=0;
  const context=vm.createContext({THREE,RoundedBoxGeometry,React,
    useRef:value=>({current:value}),useEffect(){},useState:initial=>[stateIndex++===5?avatar:initial,()=>{}]});
  let source=original.replace(/^import .*$/gm,'').replace('export default function Home','function Home');
  source+='\nglobalThis.OUTPUT = Home();';
  vm.runInContext(ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ES2022,jsx:ts.JsxEmit.React}}).outputText,context);
  return renderToStaticMarkup(context.OUTPUT);
}
function normalizeUI(html) {
  const {document}=parseHTML(`<html><body>${html}</body></html>`);
  // The Rust-owned container has display:contents, so it adds no layout box.
  const wrapper=document.querySelector('#needs-list');
  if(wrapper)wrapper.replaceWith(...wrapper.childNodes);
  for(const e of document.querySelectorAll('[id]'))e.removeAttribute('id');
  for(const e of document.querySelectorAll('em[style]')) {
    const value=e.getAttribute('style').replaceAll(' ','').replace(/;$/,'');e.setAttribute('style',value);
  }
  function tree(node) {
    if(node.nodeType===3)return node.textContent;
    if(node.nodeType!==1)return null;
    return {tag:node.localName,attributes:Object.fromEntries([...node.attributes].map(a=>[a.name,a.value]).sort()),children:[...node.childNodes].map(tree).filter(x=>x!==null)};
  }
  return tree(document.body.firstElementChild);
}
let meshCount;
for(const body of ['adulto','delgado','atlético']) {
  const avatar={...defaultAvatar,body};
  const {spec,ui}=rust({mode:'scene',avatar,random});
  const originalWorld=reference(avatar).ref;
  const rustWorld=make_scene(spec);
  close(describe(rustWorld.scene),describe(originalWorld.scene),`scene(${body})`);
  close(rustWorld.camera.position.toArray(),originalWorld.camera.position.toArray(),`camera(${body})`);
  close([spec.background,...spec.fog],[originalWorld.scene.background.getHex(),originalWorld.scene.fog.near,originalWorld.scene.fog.far],'environment');
  assert.deepEqual(normalizeUI(ui),normalizeUI(originalUI(avatar)),`UI(${body})`);
  meshCount=spec.objects.filter(o=>o.kind!=='group').length;
}
console.log(`Scene parity: ${meshCount} meshes, materials, shadows, vertex data and three body types match the original. Initial UI DOM matches.`);

const {spec}=rust({mode:'scene',random});
const world=make_scene(spec);
const bounds=bounds_for(world,spec.obstacles).filter(b=>b.min.y<1.6&&b.max.y>0.3);
const originalWorld=reference();
close(bounds,originalWorld.ref.bounds.slice(0,-3).map(b=>({min:{x:b.min.x,y:b.min.y,z:b.min.z},max:{x:b.max.x,y:b.max.y,z:b.max.z}})),'obstacles');
const ops=[];
let now=0;
function op(kind,data={}){ops.push({kind,...data,snapshot:true});}
function frames(count,dt=0.05){for(let i=0;i<count;i++){now+=dt*1000;op('tick',{dt,now});}}
op('creator',{value:false});
// Each action runs from the previous one, covering real paths around furniture.
for(const task of ['bed','tv','shower','toilet','eat']){op('command',{task});frames(850);}
op('walk',{point:{x:0.4,y:0,z:4.2}});frames(350);
op('walk',{point:{x:-4.25,y:0,z:2.7}});frames(5); // occupied bed: ignored
op('command',{task:'shower'});frames(100);op('paused',{value:true});frames(80);op('paused',{value:false});frames(700);
op('command',{task:'bed'});frames(100);op('command',{task:'tv'});frames(800); // interrupt a walk
op('creator',{value:true});frames(60);op('creator',{value:false});
for(const index of [0,1,2,3,4]){const values=[90,90,90,90,90];values[index]=2;op('values',{value:values});frames(1100);}
frames(100,0.2); // frame clamp after a long browser frame
const output=rust({bounds,random,ops});
assert.deepEqual(output.free,Array.from(originalWorld.ref.free),'Every navigation cell must match');
let previousNow=0;
for(let i=0;i<ops.length;i++) {
  const o=ops[i],r=originalWorld.ref;
  if(o.kind==='command')r.command(o.task);
  if(o.kind==='walk')r.walk(new THREE.Vector3(o.point.x,o.point.y,o.point.z));
  if(o.kind==='paused')r.control.current.paused=o.value;
  if(o.kind==='creator')r.control.current.creator=o.value;
  if(o.kind==='values')r.setValues(o.value);
  if(o.kind==='tick'){originalWorld.tick(o.now);previousNow=o.now;}
  const expected=r.snapshot();
  close(output.snapshots[i],JSON.parse(JSON.stringify(expected)),`frame ${i} (${o.kind})`,1e-9);
}
console.log(`Simulation parity: ${ops.length} operations match the original, including ${output.free.length} navigation cells, five actions, autonomy, pause, interruptions and frame clamping.`);

const escaped={...defaultAvatar,name:'<img src=x onerror="bad()">& Ñ'};
const safe=rust({mode:'scene',avatar:escaped,random});
assert.deepEqual(normalizeUI(safe.ui),normalizeUI(originalUI(escaped)),'Name escaping and Unicode');
const css=readFileSync('web/styles.css','utf8').replaceAll('\r\n','\n');
const originalCss=readFileSync('tests/reference/original-globals.css','utf8').replaceAll('\r\n','\n').replace('@import "tailwindcss";\n','');
assert.ok(css.startsWith(originalCss),'Original CSS must be preserved exactly');
console.log('Styles and responsive rules are preserved; names remain escaped.');
