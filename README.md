# Vida Isométrica 3D

Simulación de vida isométrica en Rust con Bevy, egui y MSPA-Lib. La arquitectura visual usa modelos GLB importados; la simulación, navegación, psicología e interacción ambiental permanecen separadas del contenido gráfico.

## Preparación

Instala Rust mediante rustup y los Build Tools de Visual Studio con Desarrollo para el escritorio con C++ y Windows SDK. MSPA-Lib se compila estáticamente con el proyecto.

Los modelos binarios se cargan directamente desde `assets/`. Consulta `ASSETS.md` para la estructura exacta. Los GLB conservan malla, UV, rig y animaciones; sólo se reducen texturas grandes cuando es necesario para memoria de ejecución.

```powershell
git pull origin main
cargo build --release
.\target\release\mens-life.exe
```

## Casa y cámara

La escena usa la casa Autumn House suministrada, con vivienda, veranda, patio, terreno exterior, vegetación y elementos de la propiedad. No se reconstruye con primitivas procedurales.

La cámara es ortográfica y puede rotarse con:

- Q / Flecha izquierda: girar a la izquierda.
- E / Flecha derecha: girar a la derecha.
- Botón derecho del ratón + arrastre horizontal: rotación continua.

Cuando la residente seleccionada está dentro, el techo y el muro situado entre la cámara y el interior se ocultan según el ángulo actual. Los grupos `CUTAWAY_ROOF`, `CUTAWAY_EAST`, `CUTAWAY_WEST`, `CUTAWAY_NORTH` y `CUTAWAY_SOUTH` pertenecen al modelo importado; no sustituyen su geometría.

La entrada conecta la navegación interior con veranda, patio y exterior. Las colisiones invisibles siguen las dimensiones reales del GLB y dejan abierta la puerta real.

## Personajes

El selector muestra únicamente los cinco GLB de personaje disponibles:

- Mia
- Kurumi
- Mai
- Rin
- Yui

La referencia de la chica de camiseta rosada queda reservada, pero no se sustituye por otro modelo: todavía no se dispone de su archivo 3D correspondiente.

Los cinco modelos se normalizan mediante transformaciones de escena para quedar aproximadamente entre 1,65 y 1,67 m de altura. No se deciman sus mallas.

Kurumi y Mai conservan clips de reposo y marcha. Los modelos que incluyen otros clips conservan su rig y animaciones originales. Cuando un GLB no trae una marcha utilizable se aplica sólo un movimiento raíz mínimo como fallback; no se reconstruye el cuerpo.

La chica elegida vive dentro de la casa. Las demás existen simultáneamente como residentes y pasean por el exterior sin atravesar el interior.

## MSPA por personaje

Cada chica posee su propio perfil MSPA. El perfil visual y la identidad psicológica están separados para evitar mezclar estados o rasgos al cambiar de personaje.

El panel `MSPA · rasgos por personaje` permite inspeccionar cualquier residente y muestra directamente desde MSPA:

- sociabilidad, estabilidad, empatía, persistencia, curiosidad y audacia;
- rigidez de creencias, persistencia de metas, resistencia al cambio, sensibilidad a amenaza, sensibilidad social y exploración;
- emociones, valencia, activación y confianza;
- recuerdos, creencias y patrones aprendidos.

Las personalidades son diferentes. Kurumi tiene un perfil más mordaz, impulsivo y confrontacional; Yui es más dulce, empática y sociable, aunque también impulsiva. Mai es más estable y reservada. Las diferencias modifican decisiones sociales y reacciones a fenómenos de la casa.

## Agencia invisible e interacción

El usuario no controla directamente a las residentes. Las personas toman decisiones mediante su simulación y MSPA.

Los clics sobre elementos de la casa representan una intervención ambiental anónima. La simulación no entrega a MSPA una identidad causal de “jugador”. Por ejemplo:

- la televisión puede encenderse sola;
- la ducha puede abrirse sin nadie dentro;
- la cama puede hundirse;
- la cisterna puede sonar;
- la vajilla puede moverse o golpear la encimera.

Las chicas perciben el resultado, lo incorporan a su estado MSPA y reaccionan según personalidad, amenaza percibida, exploración y experiencia previa.

## Progresión sobrenatural idle

La presencia sobrenatural acumula esencia con el tiempo incluso sin interacción. La actividad en la casa y las conversaciones aceleran ligeramente el progreso; las intervenciones ambientales también aportan esencia.

Hay 20 niveles desbloqueables:

1. Crujidos
2. Golpes lejanos
3. Pasos sin dueño
4. Objetos desplazados
5. Susurros
6. Voces lejanas
7. Voces que responden
8. Sombras periféricas
9. Luces inestables
10. Puertas inquietas
11. Frío localizado
12. Reflejos imposibles
13. Presencia
14. Siluetas fugaces
15. Contacto invisible
16. Apariciones parciales
17. Espectro tenue
18. Espectros móviles
19. Interacción espectral
20. Manifestación completa

El HUD muestra nivel, progreso, siguiente desbloqueo y fenómeno reciente. Los poderes ya desbloqueados pueden recorrerse y manifestarse manualmente. Desde el nivel 9 aparecen efectos de iluminación; desde el 17 pueden hacerse visibles espectros y los niveles finales aumentan su duración y presencia.

Los eventos no producen una reacción genérica. MSPA condiciona si cada personaje responde con miedo, curiosidad, ansiedad, escepticismo o desafío, y la presentación utiliza una voz distinta por perfil.

## Mobiliario

Los muebles visibles son los GLB suministrados: cama, sofá, mesas, sillas y set de baño. Sus tamaños se normalizan mediante transformaciones de escena; los proxies invisibles sólo sirven para navegación e interacción.

El set adicional `furniture_set.glb` permanece disponible en `assets/furniture/` para ampliar decoración sin mezclarlo con la lógica de la casa.

## Organización del código

- `src/native.rs`: ensamblaje de la aplicación Bevy.
- `src/native_scene.inc.rs`: carga de GLB y composición runtime.
- `src/native_runtime.inc.rs`: cámara, cutaway, animaciones y actualización visual.
- `src/native_interaction.inc.rs`: selección e interacción 3D.
- `src/scene.rs`: especificación de proxies y efectos ligeros.
- `src/house.rs`: colisiones de la arquitectura real.
- `src/furnishings.rs`: proxies de mobiliario importado.
- `src/character_catalog.rs`: catálogo, escala, animaciones y perfil de cada chica.
- `src/character_ui.rs`: selector de personaje.
- `src/society.rs`: residentes exteriores, relaciones y encuentros MSPA.
- `src/psychology.rs`: propiedad Rust del runtime MSPA.
- `src/mspa_ui.rs`: visor de rasgos por personaje.
- `src/perception.rs`: percepción anónima de cambios ambientales.
- `src/poltergeist.rs`: intervenciones ambientales sin actor identificado.
- `src/poltergeist_ui.rs`: controles de intervención de la casa.
- `src/haunting.rs`: progresión sobrenatural de 20 niveles.
- `src/haunting_ui.rs`: progreso y selector de poderes.
- `src/game.rs`: necesidades, decisiones autónomas y navegación.
- `native/mspa_bridge.cpp`: puente C++ y perfiles MSPA.
- `vendor/MSPA-Lib`: núcleo MSPA-Lib.

## Estado de sesión

La psicología y la progresión sobrenatural se mantienen durante la ejecución actual. El guardado persistente y la progresión offline todavía no están implementados.

Consulta `THIRD_PARTY.md` para dependencias y licencias.
