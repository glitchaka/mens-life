# Imported 3D assets

The runtime deliberately loads the user-supplied GLB files directly from `assets/`; it does not reconstruct them into procedural meshes.

Expected paths:

- `assets/house/house.glb`
- `assets/characters/original_girl.glb`
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

The house asset preserves the supplied house/property geometry and UV/material layout. Wall and roof pieces are separated only into visibility groups so the isometric cutaway can hide the camera-facing shell while a character is indoors.

Character/furniture geometry, skinning and animation data are preserved. Asset sizing is handled by scene transforms. Large embedded textures may be resized for runtime memory use without decimating the meshes.
