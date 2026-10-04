# Vida Isométrica 3D / Mens Life

Migración de [Vida Isométrica 3D](https://vida-isometrica-3d.butt-clapdev.chatgpt.site) a Rust, conservando la habitación, el personaje, la interfaz y las reglas del proyecto original.

Rust construye los 130 objetos de la escena y la interfaz, y ejecuta las necesidades, autonomía, navegación, acciones, animaciones y eventos. La versión web compila ese código a WebAssembly. El adaptador `web/bridge.js` conecta los datos de Rust con Three.js 0.180.0, el mismo renderizador del original; no contiene la lógica del juego. React y TypeScript se usan únicamente para ejecutar la referencia en las pruebas, no se distribuyen con el juego.

El ejecutable de escritorio para Windows incluye el WebAssembly y todos los recursos y los muestra mediante WebView2. No necesita Node ni conexión a Internet para jugar. Requiere Windows 10/11 x64 con Microsoft Edge WebView2 Runtime instalado. Al distribuir la compilación GNU, conserva `WebView2Loader.dll` junto a `mens-life.exe`.

## Compilación

Instala [Rust mediante rustup](https://rustup.rs/) y Node.js 22 o superior. `rust-toolchain.toml`, `Cargo.lock` y `package-lock.json` fijan las herramientas y dependencias.

```sh
npm ci
npm run setup
npm run build
npm run verify
```

`npm run setup` prepara Rust 1.90.0, el destino `wasm32-unknown-unknown`, rustfmt, Clippy y wasm-bindgen CLI 0.2.104. `npm run build` produce `dist/`, listo para un servidor HTTP estático o Sites. La compilación web no necesita MinGW. No abras `index.html` con `file://`, porque los navegadores necesitan HTTP para cargar WebAssembly.

En Windows, instala además las herramientas C++ de Visual Studio y Windows SDK, que usa el destino Rust MSVC predeterminado:

```sh
npm run build:windows
```

El resultado está en `artifacts/mens-life-windows-x64/`. En Linux se puede compilar el mismo ejecutable con `x86_64-pc-windows-gnu`, instalando MinGW-w64; el script añade ese destino automáticamente. La toolchain GNU se usa para la compilación cruzada en Linux, no es un requisito del proyecto web ni de Rust en Windows con MSVC.

GitHub Actions compila y verifica la versión web y la de Windows en cada cambio a `main`. Sus paquetes se descargan desde [Actions](https://github.com/glitchaka/mens-life/actions) al abrir una ejecución terminada y sus artefactos.

## Controles

Al entrar, crea el personaje con nombre, complexión y colores. Pulsa **ENTRAR A LA CASA**. Los botones inferiores indican comer, ver televisión, dormir, ducharse o usar el baño. También puedes pulsar los muebles o un punto libre del suelo. La persona atiende de forma autónoma la necesidad más baja. El botón superior pausa o reanuda; el icono de persona abre de nuevo el creador.

Se conservan las reglas del original: editar el personaje reconstruye la escena y reinicia la simulación; no hay guardado persistente ni controles de velocidad adicionales.

## Comprobación frente al original

`tests/reference/` conserva el código de referencia del commit de Sites `eea5ead940413b4eb92b9381e1d2b9fb4f4b7a0a`. La verificación compara el código original con la biblioteca Rust real y ejecuta también el WebAssembly generado:

- Los 130 objetos: geometrías, datos de vértices, transformaciones, materiales, iluminación y sombras, en las tres complexiones.
- Las 3.621 celdas de navegación y las rutas alrededor de los muebles.
- Más de 12.000 operaciones de simulación: las cinco acciones, necesidades, autonomía, pausas, interrupciones y límite de tiempo por fotograma.
- DOM inicial y estilos originales, nombres escapados y Unicode.
- Eventos de la interfaz compilada: botones, selección de muebles mediante ray casting, creador, colores y sustitución del canvas al editar.

Estas pruebas ejecutan Three.js y el WebAssembly en Node con un DOM de prueba. No dibujan mediante GPU y no sustituyen una comparación de capturas en un navegador real. El ejecutable Windows se ha compilado y empaquetado desde Linux; su ejecución nativa necesita Windows y WebView2.

## Código

| Archivo | Función |
| --- | --- |
| `src/game.rs` | Simulación, navegación y animaciones |
| `src/scene.rs` | Geometrías, materiales, luces y avatar |
| `src/ui.rs` | Interfaz y textos |
| `src/web.rs` | Eventos DOM y bucle WebAssembly |
| `src/desktop.rs` | Ventana Windows y recursos incluidos |
| `web/bridge.js` | Adaptador del motor gráfico |
| `web/styles.css` | Diseño original y reglas responsivas |
| `scripts/build.mjs` | Compilación y empaquetado reproducibles |
| `tests/parity.mjs` | Comparación con el código original |
| `tests/wasm-ui.mjs` | Ejecución de la interfaz WebAssembly |

Las licencias de Three.js y del CSS de Tailwind se copian a `dist/vendor/` al compilar.
