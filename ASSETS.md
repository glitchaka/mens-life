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

`assets/house/house.glb` is the supplied `(PSX) - Backroom` model by Kasuga. Its own glTF node transforms already contain the Sketchfab/FBX unit conversion, so the runtime keeps the model at `scale = 1.0` and preserves its original mesh, UVs, materials and embedded textures.

Measured after those internal transforms, the model occupies approximately `46.8974 × 55.0137 m` in X/Z and `4.9106 m` vertically. The scene root translates it by `(-3.195875, +1.246176, +20.446539)` so the footprint is centred on X/Z and the lowest floor geometry sits at `Y = 0`.

The raw GLB is intentionally not rehosted by this public repository. During a local build, `build.rs` validates the exact supplied asset (1,110,456 bytes and GLB header) and automatically installs it from the first valid location found among:

- `MENS_LIFE_HOUSE_GLB`
- the repository root
- the user's `Downloads` directory
- the user's `Desktop` directory

Accepted local filenames are `psx_-_backroom.glb`, `psx-backroom.glb`, and `house.glb`.

The model itself is the visible architecture. The old Two Stories / Autumn House shell is no longer rendered. The three top meshes (`Backroom_lambert5_0`, `Backroom_lambert6_0`, `Backroom_lambert7_0`) are treated as the ceiling cutaway while the resident is indoors. Gameplay furniture and house boundaries remain lightweight invisible interaction/navigation proxies only.

Character and furniture geometry, skinning and animation data are preserved. Asset sizing is handled by scene transforms. Large embedded textures may be resized for runtime memory use without decimating meshes, changing UVs, removing bones or stripping animation clips.
