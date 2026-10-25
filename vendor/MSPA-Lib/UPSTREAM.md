# MSPA-Lib vendorizada

Origen: https://github.com/glitchaka/MSPA-Lib.git

Base: `4532daf43a4ffa0a2b7abb8cace29ba53a9b8d89` (v1.0.0).

Backport local: invariante de baseline emocional de MSPA 1.1.1. La copia vendorizada se identifica como `1.0.1` porque no incluye todavía el resto de la API 1.1.x.

Todo agente activo conserva las seis emociones base con un tono mínimo distinto de cero; el decaimiento vuelve al baseline en vez de borrar la emoción. La UI puede leer `AgentPsychState::emotions` directamente.

El adaptador específico del juego está en `native/mspa_bridge.cpp`; Cargo compila el núcleo como biblioteca estática mediante `build.rs`.
La copia permite compilar tras un clon normal, sin submódulos ni descargas adicionales de código durante el build.
