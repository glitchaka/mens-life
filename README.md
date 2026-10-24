# Vida Isométrica 3D — Rust nativo para Windows

Juego de escritorio en Rust. Bevy dibuja las mallas 3D, materiales, sombras,
cámara y animaciones. egui dibuja los controles dentro de la misma ventana.
La casa, el personaje y las fuentes se incluyen en el ejecutable.

## Compilar

Instala [Rust con rustup](https://rustup.rs/) y los Build Tools de Visual Studio
con **Desarrollo para el escritorio con C++** y Windows SDK, necesarios para
compilar MSPA-Lib y para el enlazador de la toolchain MSVC.

```powershell
git pull origin main
cargo build --release
.\target\release\mens-life.exe
```

**`cargo build --release` genera `target/release/mens-life.exe`.**
El ejecutable es el destino predeterminado de Cargo. No requiere opciones de
features ni pasos de npm, Node, WebAssembly o generación de una web.
La primera compilación descarga y compila las dependencias del motor.
`rust-toolchain.toml` fija Rust 1.90.0 con perfil mínimo y `Cargo.lock` fija las
versiones de las dependencias.

Para compilar y abrir el juego directamente:

```powershell
cargo run --release
```

El ejecutable distribuido está preparado para Windows 10/11 de 64 bits.
Necesita una GPU con controladores compatibles con DirectX 12 o Vulkan.

## Jugar

Elige nombre, complexión y colores y pulsa **ENTRAR A LA CASA**. El personaje
atiende sus necesidades automáticamente. Pulsa el suelo para caminar, un
mueble para usarlo o Comer, Televisión, Dormir, Ducha y Baño para indicarle
una actividad. El lavabo activa Comer, igual que en la referencia.

El botón de pausa y la barra espaciadora pausan o reanudan la simulación.
El botón del personaje y Escape abren o cierran el creador.

## Personajes e interacción

El protagonista, Mara y Nicolás usan un modelo humano de aproximadamente
1,89 unidades: cabeza proporcionada, mandíbula, nariz, orejas, manos con
dedos, ropa y un esqueleto con caderas, rodillas, hombros y codos.
La marcha articula las piernas y brazos; el reposo incluye respiración y
parpadeo. El modelo es procedural y se incluye en el ejecutable.

El panel **EN CASA** permite saludar, hablar o bromear con Mara y Nicolás.
También se puede pulsar su cuerpo para seleccionarlos. El protagonista se
acerca por un camino libre, los dos se miran, gesticulan y alternan frases.
Los residentes caminan por la casa y pueden iniciar encuentros cercanos
entre ellos o con el protagonista. Las conversaciones completadas mejoran
el afecto, la confianza y la familiaridad en ambas direcciones, usando los
estados y reportes reales de MSPA. Las órdenes de movimiento o de muebles
interrumpen el encuentro. La pausa y el creador detienen a todos.

La posición navegable se conserva separada de la pose animada. La cama tiene
el personaje boca arriba, con la cabeza sobre la almohada; la postura de
sentado flexiona rodillas y caderas. Al terminar se anima la salida hasta el
punto libre de entrada. Una orden durante el uso parte de ese mismo punto,
evitando intentar caminar desde el interior del váter, la ducha o la cama.
El acceso del baño es más ancho y los obstáculos salen de las mallas reales.

## Psicología del personaje

[MSPA-Lib](https://github.com/glitchaka/MSPA-Lib) está integrada y enlazada
estáticamente en el ejecutable. Su núcleo C++17 se compila automáticamente
con Cargo; no requiere una DLL externa, CMake ni inicializar submódulos.

El juego comunica las necesidades percibidas mediante `OBSERVE`, ofrece
únicamente actividades a las que se puede llegar y usa `DECIDE` para la
autonomía. MSPA considera necesidades, valores, metas, personalidad,
experiencias aprendidas y continuidad de la actividad. También puede elegir
descansar si las necesidades están cubiertas. Las órdenes del jugador se
respetan hasta terminar o hasta recibir otra indicación.

Los resultados físicos de comer, dormir, ducharse, entretenerse o ir al baño
se comunican con `REPORT`. Los caminos bloqueados producen un fallo; una
interrupción del jugador se observa sin inventar un fracaso. Las emociones y
el ánimo avanzan con el tiempo de simulación y se congelan durante la pausa
y el creador. Cambiar el nombre o la apariencia conserva la psicología.

Una burbuja sobre la cabeza muestra una carita y frases breves: «Me ruge el
estómago», «Zzz...», «¡Qué alivio!» o reacciones al ánimo del personaje.
El panel sobre las necesidades muestra el emoticono y el estado actual;
al posar el cursor muestra recuerdos, creencias y experiencias aprendidas.
Las caritas se dibujan con vectores para que funcionen sin fuentes emoji
instaladas. Las frases en español pertenecen a la presentación del juego:
no se atribuyen a un generador de lenguaje dentro de MSPA.

La psicología se conserva durante la sesión; todavía no hay guardado de
partidas entre ejecuciones. La revisión de MSPA incluida es
`4532daf43a4ffa0a2b7abb8cace29ba53a9b8d89`.

## Código

- `src/main.rs` y `src/native.rs`: aplicación, GPU e interacción 3D.
- `src/geometry.rs` y `src/scene.rs`: habitación y personaje articulado.
- `src/human.rs`: superficies anatómicas, ropa y esqueleto humano.
- `src/society.rs`: residentes, encuentros físicos y conversaciones recíprocas.
- `src/game.rs`: necesidades, autonomía, navegación y actividades.
- `src/psychology.rs`: propietario Rust del runtime y traducción de estados a expresiones.
- `native/mspa_bridge.cpp`: Domain Pack de la casa y puente con el núcleo MSPA real.
- `vendor/MSPA-Lib`: núcleo original, sin modificaciones y con revisión fijada.
- `build.rs`: compilación estática de MSPA como parte de Cargo.
- `src/ui.rs`: creador y controles dibujados en Rust.
- `tests/`: navegación original, controles y escenarios de integración del personaje.

El workflow de GitHub Actions se inicia manualmente y solo compila y adjunta
el ejecutable de Windows. No ejecuta validaciones automáticas ni genera
versiones para otros sistemas operativos.

Consulta `THIRD_PARTY.md` para las dependencias y sus licencias.
