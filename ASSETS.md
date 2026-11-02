# Imported 3D assets

The runtime loads the GLB files directly from `assets/`; it does not reconstruct or decimate them into procedural meshes.

Expected paths:

- `assets/house/house.glb` — `(PSX) - Backroom`, complete visible house interior.
- `assets/characters/mia.glb`
- `assets/characters/kurumi.glb`
- `assets/characters/mai.glb`
- `assets/characters/outfits.glb`
- `assets/characters/yui.glb`
- `assets/furniture/classic_table.glb`
- `assets/furniture/chair.glb`
- `assets/furniture/bathroom.glb`
- `assets/furniture/sofa.glb`
- `assets/furniture/small_table.glb`
- `assets/furniture/bed.glb`
- `assets/furniture/furniture_set.glb`

The standing pink-shirt/blue-jeans reference character is deliberately not listed: only a reference image is available, not its actual 3D model.

## House interior

`assets/house/house.glb` is `(PSX) - Backroom` by Kasugay. Its glTF node transforms already contain the Sketchfab/FBX unit conversion, so the runtime keeps the model at `scale = 1.0` and preserves its original mesh, UVs, materials and embedded textures.

The model is centred on X/Z. The actual carpet/floor mesh (`Backroom_lambert2_0`) is aligned to simulation `Y = 0`; characters no longer use the lower decorative geometry as their floor reference.

The Windows artifact **contains** `assets/house/house.glb`. A source checkout runs `tools/ensure-assets.ps1` before compiling and obtains the pinned project asset directly from an immutable source revision when it is missing. The build never scans `Downloads`, `Desktop`, environment-specific personal folders, or arbitrary local GLB files.

`build.rs` only validates the packaged `assets/house/house.glb`; it does not search the user's filesystem.

The old Two Stories / Autumn House assets were removed. The Backroom is the only visible house architecture. The three top meshes (`Backroom_lambert5_0`, `Backroom_lambert6_0`, `Backroom_lambert7_0`) are treated as the ceiling cutaway while the resident is indoors.

Character geometry, skinning and animation data are preserved. Asset sizing is handled by scene transforms.
