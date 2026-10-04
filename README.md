# Vida Isométrica 3D — Rust nativo para Windows

Juego de escritorio en Rust. Bevy dibuja las mallas 3D, materiales, sombras,
cámara y animaciones. egui dibuja los controles dentro de la misma ventana.
La casa, el personaje y las fuentes se incluyen en el ejecutable.

## Compilar

Instala [Rust con rustup](https://rustup.rs/) y los Build Tools de Visual Studio
con **Desarrollo para el escritorio con C++** y Windows SDK, necesarios para
el enlazador de la toolchain MSVC.

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

## Código

- `src/main.rs` y `src/native.rs`: aplicación, GPU e interacción 3D.
- `src/geometry.rs` y `src/scene.rs`: habitación y personaje articulado.
- `src/game.rs`: necesidades, autonomía, navegación y actividades.
- `src/ui.rs`: creador y controles dibujados en Rust.
- `tests/`: datos de regresión del proyecto original.

El workflow de GitHub Actions se inicia manualmente y solo compila y adjunta
el ejecutable de Windows. No ejecuta validaciones automáticas ni genera
versiones para otros sistemas operativos.

Consulta `THIRD_PARTY.md` para las dependencias y sus licencias.
