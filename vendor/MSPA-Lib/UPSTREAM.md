# MSPA-Lib vendorizada

Origen: https://github.com/glitchaka/MSPA-Lib.git

Base: `9b217dc387a3b0e0152cac161da2ef44ff3677cb` (v1.1.1).

La copia vendorizada usa el snapshot oficial 1.1.1. Ya no mantiene el backport local 1.0.1: los archivos de biblioteca modificados desde la base 1.0.0 se sincronizaron con upstream.

MSPA 1.1 incorpora perfiles psicológicos definidos por el host mediante `PsychologicalProfile`, `ProfileRegistry` y `ProfileRecipe`, sin presets rígidos dentro del núcleo. MSPA 1.1.1 añade el baseline emocional tónico: todo agente activo conserva las seis emociones base por encima de un mínimo distinto de cero y el decaimiento vuelve a ese baseline.

El adaptador específico del juego permanece en `native/mspa_bridge.cpp`; Cargo compila recursivamente todos los `.cpp` de `vendor/MSPA-Lib/src` mediante `build.rs`, por lo que las nuevas unidades `profile.cpp` y `profile_runtime.cpp` forman parte del enlace estático sin una lista manual adicional.

La copia permite compilar tras un clon normal, sin submódulos ni descargas adicionales de código durante el build.
