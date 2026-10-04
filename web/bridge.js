// GPU and DOM adapter only. Scene descriptions, game rules, pathfinding, state,
// animation transforms and interface events are supplied by Rust/WebAssembly.
import * as THREE from './vendor/three.module.js';
import { RoundedBoxGeometry } from './vendor/RoundedBoxGeometry.js';

const geometries = {
  'rounded-box': RoundedBoxGeometry, box: THREE.BoxGeometry,
  cylinder: THREE.CylinderGeometry, sphere: THREE.SphereGeometry,
  capsule: THREE.CapsuleGeometry, cone: THREE.ConeGeometry,
  octahedron: THREE.OctahedronGeometry,
};
const materialTypes = {
  standard: THREE.MeshStandardMaterial,
  physical: THREE.MeshPhysicalMaterial,
  basic: THREE.MeshBasicMaterial,
};
function material(spec) {
  const { type, ...properties } = spec;
  return new materialTypes[type](properties);
}
export function make_scene(spec) {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(spec.background);
  scene.fog = new THREE.Fog(spec.background, ...spec.fog);
  const c = spec.camera;
  const camera = new THREE.OrthographicCamera(...c.bounds, c.near, c.far);
  camera.position.set(...c.position);
  camera.lookAt(...c.look_at);
  const nodes = new Map();
  const materials = new Map(Object.entries(spec.materials).map(([id, value]) => [id, material(value)]));
  function addLight(s) {
    let light;
    if (s.kind === 'hemisphere') light = new THREE.HemisphereLight(s.sky, s.ground, s.intensity);
    if (s.kind === 'directional') light = new THREE.DirectionalLight(s.color, s.intensity);
    if (s.kind === 'point') light = new THREE.PointLight(s.color, s.intensity, s.distance);
    if (s.position) light.position.set(...s.position);
    if (s.cast_shadow) light.castShadow = true;
    if (s.shadow_size) light.shadow.mapSize.set(...s.shadow_size);
    scene.add(light);
  }
  spec.lights.filter(s => s.after_objects === undefined).forEach(addLight);
  spec.objects.forEach((s, i) => {
    spec.lights.filter(l => l.after_objects === i).forEach(addLight);
    const node = s.kind === 'group' ? new THREE.Group()
      : new THREE.Mesh(new geometries[s.kind](...s.args),
        s.material_ref ? materials.get(s.material_ref) : material(s.material));
    node.name = s.name || '';
    node.position.set(...s.position);
    if (s.rotation) node.rotation.set(...s.rotation);
    if (s.scale) node.scale.set(...s.scale);
    if (s.cast_shadow) node.castShadow = true;
    if (s.receive_shadow) node.receiveShadow = true;
    if (s.visible !== undefined) node.visible = s.visible;
    nodes.set(s.id, node);
    (s.parent ? nodes.get(s.parent) : scene).add(node);
  });
  return { scene, camera, nodes };
}
export function bounds_for(world, ids) {
  return ids.map(id => {
    const b = new THREE.Box3().setFromObject(world.nodes.get(id));
    return { min: { x: b.min.x, y: b.min.y, z: b.min.z }, max: { x: b.max.x, y: b.max.y, z: b.max.z } };
  });
}
export function update_world(world, updates) {
  for (const u of updates) {
    const node = world.nodes.get(u.id);
    if (u.position) node.position.set(...u.position);
    if (u.y !== undefined) node.position.y = u.y;
    if (u.rotation) node.rotation.set(...u.rotation);
    if (u.visible !== undefined) node.visible = u.visible;
    if (u.material) {
      if (u.material.color !== undefined) node.material.color.set(u.material.color);
      if (u.material.emissive_hsl) node.material.emissive.setHSL(...u.material.emissive_hsl);
    }
  }
}
let active;
export function create_renderer(host, json) {
  dispose_renderer();
  const spec = JSON.parse(json);
  const world = make_scene(spec);
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(Math.min(devicePixelRatio, 2));
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  host.appendChild(renderer.domElement);
  const resize = () => {
    const w = host.clientWidth, h = host.clientHeight;
    if (!w || !h) return;
    const a = w / h, v = Math.max(spec.camera.resize.min_vertical, spec.camera.resize.min_horizontal / a);
    const c = world.camera;
    c.left = -v * a; c.right = v * a; c.top = v; c.bottom = -v;
    c.updateProjectionMatrix();
    renderer.setSize(w, h);
  };
  resize();
  const observer = new ResizeObserver(resize);
  observer.observe(host);
  active = { ...world, renderer, observer };
  return JSON.stringify(bounds_for(world, spec.obstacles));
}
export function apply_updates(json) { update_world(active, JSON.parse(json)); }
export function render_frame() { active.renderer.render(active.scene, active.camera); }
export function pick_scene(x, y) {
  const rect = active.renderer.domElement.getBoundingClientRect();
  const pt = new THREE.Vector2((x-rect.left)/rect.width*2-1, -((y-rect.top)/rect.height)*2+1);
  const ray = new THREE.Raycaster();
  ray.setFromCamera(pt, active.camera);
  const names = ray.intersectObjects(active.scene.children, true).map(h => h.object.name);
  const hit = ray.intersectObject(active.nodes.get('floor'))[0];
  return JSON.stringify({ names, floor: hit ? {x:hit.point.x,y:hit.point.y,z:hit.point.z} : null });
}
export function dispose_renderer() {
  if (!active) return;
  active.observer.disconnect();
  const mats = new Set();
  active.scene.traverse(o => {
    if (o instanceof THREE.Mesh) {
      o.geometry.dispose();
      (Array.isArray(o.material) ? o.material : [o.material]).forEach(m => mats.add(m));
    }
  });
  mats.forEach(m => m.dispose());
  active.renderer.dispose();
  active.renderer.domElement.remove();
  active = undefined;
}
