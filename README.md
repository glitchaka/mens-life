# Vida Isométrica 3D — Rust nativo

Juego de escritorio en Rust. Bevy dibuja las mallas 3D, materiales, sombras,
cámara y animaciones. egui dibuja los controles dentro de la misma ventana.
La casa y el personaje se generan en Rust y las fuentes se incluyen en el
binario; puedes ejecutar el juego sin conexión y sin archivos de recursos.

## Compilar en Windows

Instala [Rust con rustup](https://rustup.rs/) y los **Build Tools de Visual
Studio**, con la carga **Desarrollo para el escritorio con C++** y Windows
SDK, que necesita el enlazador de la toolchain MSVC.

Desde este repositorio:

```powershell
git pull origin main
cargo build --release
.\target\release\mens-life.exe
```

**`cargo build --release` genera `target/release/mens-life.exe`.** El binario
es un destino predeterminado de Cargo y no necesita ninguna opción adicional.
La primera compilación descarga y compila las dependencias del motor.
`rust-toolchain.toml` fija Rust 1.90.0 y `Cargo.lock` fija las dependencias.
No hay pasos de npm, Node, WebAssembly ni generación de una web.

También puedes compilar y abrir el juego directamente:

```powershell
cargo run --release
```

## Linux

Requiere un entorno gráfico X11 y un controlador compatible con Vulkan.
En Ubuntu/Debian instala las dependencias de desarrollo y ejecuta Cargo:

```sh
sudo apt-get install build-essential libx11-dev libxkbcommon-dev libxi-dev libxrandr-dev mesa-vulkan-drivers
cargo run --release
```

El ejecutable queda en `target/release/mens-life`.
En Windows el motor utiliza la GPU mediante DirectX 12 o Vulkan.

## Jugar

Elige el nombre, la complexión y los colores en el creador del personaje.
Pulsa **ENTRAR A LA CASA** para empezar. El personaje atiende sus necesidades
automáticamente. Pulsa el suelo para caminar, un mueble para usarlo o los
botones Comer, Televisión, Dormir, Ducha y Baño para darle una indicación.
El lavabo activa Comer, igual que en la referencia.

El botón de pausa y la barra espaciadora pausan/reanudan la simulación.
El botón del personaje y Escape abren/cierran el creador.

## Comprobación

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
```

Los tests comparan las 3.621 celdas de navegación y una secuencia de 12.065
operaciones contra datos capturados del proyecto original: cinco actividades,
autonomía, pausas, interrupciones, caminar y límites del tiempo por fotograma.
Los controles nativos también se comprueban con eventos reales de egui.
Estos tests validan los estados y las rutas; no son una comparación de píxeles
entre motores gráficos.

Para guardar una captura del renderizador nativo, con la misma ventana y GPU
que utiliza el juego:

```sh
cargo run --release -- --capture creator.png
cargo run --release -- --capture house.png --start
cargo run --release -- --capture shower.png --task shower --simulate 400
```

El juego guarda el PNG tras preparar los shaders y cierra la ventana.
GitHub Actions compila y verifica el código en Windows MSVC y Linux, comprueba
que existe el `.exe` y adjunta los ejecutables a la ejecución del workflow.

## Código

- `src/main.rs` y `src/native.rs`: aplicación de escritorio, GPU e interacción 3D.
- `src/geometry.rs` y `src/scene.rs`: mallas, habitación y personaje articulado.
- `src/game.rs`: necesidades, autonomía, navegación y actividades.
- `src/ui.rs`: creador y controles dibujados en Rust.
- `tests/`: regresión de comportamiento y controles nativos.

Consulta `THIRD_PARTY.md` para las dependencias y sus licencias.
