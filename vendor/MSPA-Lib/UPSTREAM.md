# MSPA-Lib vendorizada

Origen: https://github.com/glitchaka/MSPA-Lib.git

Revisión: `4532daf43a4ffa0a2b7abb8cace29ba53a9b8d89` (v1.0.0).

Los archivos de `include/` y `src/` son una copia sin modificaciones.
El adaptador específico del juego está en `native/mspa_bridge.cpp`;
Cargo compila el núcleo como biblioteca estática mediante `build.rs`.
La copia permite compilar tras un clon normal, sin submódulos ni descargas
adicionales de código durante el build.
