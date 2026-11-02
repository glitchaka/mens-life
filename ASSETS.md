# Imported 3D assets

The runtime deliberately loads the user-supplied GLB files directly from `assets/`; it does not reconstruct or decimate them into procedural meshes.

Expected paths:

- `assets/house/house.glb` — `(PSX) - Backroom`, used as the complete visible house interior.
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

The standing pink-shirt/blue-jeans reference character is deliberately not listed: only a reference image is available, not its actual 3D model. The rejected kneeling OBJ is never substituted for it.

## House interior

`assets/house/house.glb` is the supplied PSX Backroom model by Kasuga. The runtime keeps its original mesh, UVs, materials and textures. It is transformed as one scene root with a uniform `0.1` world scale, centred around the simulation origin, and its carpet/floor plane is aligned to `Y = 0`.

The model itself is the visible architecture. The old Two Stories / Autumn House shell is no longer rendered. Gameplay furniture and house boundaries remain lightweight invisible interaction/navigation proxies only.

Character and furniture geometry, skinning and animation data are preserved. Asset sizing is handled by scene transforms. Large embedded textures may be resized for runtime memory use without decimating meshes, changing UVs, removing bones or stripping animation clips.
