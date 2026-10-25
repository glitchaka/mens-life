# Dependencias

MSPA-Lib 1.1.1, de `glitchaka/MSPA-Lib`, revisión
`9b217dc387a3b0e0152cac161da2ef44ff3677cb`, se incluye en
`vendor/MSPA-Lib` y se enlaza estáticamente. Su origen y revisión están
en `vendor/MSPA-Lib/UPSTREAM.md`. Esa revisión del repositorio original no
incluye un archivo de licencia; se conserva su atribución y documentación.
El adaptador del juego queda fuera de la biblioteca original, en `native/`.

El juego usa Bevy 0.17.3 (MIT / Apache-2.0), wgpu (MIT / Apache-2.0),
winit (Apache-2.0), egui y bevy_egui (MIT / Apache-2.0 / MIT).
La interfaz incorpora subconjuntos de Inter (SIL Open Font License 1.1) y
DejaVu Sans (Bitstream Vera / DejaVu), con los textos de licencia en
`assets/fonts/`. Las fuentes predeterminadas de egui incluyen Ubuntu (Ubuntu Font Licence)
y Hack (MIT / Bitstream Vera). Se integran mediante la dependencia egui.

El grafo completo de dependencias y versiones está en `Cargo.lock`.
Sus licencias se conservan en los paquetes publicados por cada autor.

Geometría, distribución de la casa, colores, textos y comportamiento se
adaptan del proyecto Vida Isométrica 3D indicado por el propietario de
este repositorio. Los datos de regresión proceden de esa misma referencia.
