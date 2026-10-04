# Dependencias

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
