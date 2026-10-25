# Imported 3D assets

The runtime deliberately loads the user-supplied GLB files directly from `assets/`; it does not reconstruct or decimate them into procedural meshes.

Expected paths:

- `assets/house/house.glb`
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

The house asset preserves the supplied house/property geometry, patio, UV/material layout and hierarchy. Wall and roof pieces are separated only into visibility groups so the cutaway can hide the camera-facing shell while the selected resident is indoors.

Character and furniture geometry, skinning and animation data are preserved. Asset sizing is handled by scene transforms. Large embedded textures may be resized for runtime memory use without decimating meshes, changing UVs, removing bones or stripping animation clips.
