# Proyecto 3: Renderizador Sistema solar

## Descripción

Este proyecto, desarrollado en Rust con la librería Raylib, genera un sistema solar interactivo mediante un motor de renderizado 3D personalizado. El sistema simula múltiples cuerpos celestes orbitando alrededor de una estrella central, cada uno representado con shaders únicos y realistas.

### Características Principales

- **Sistema Solar Dinámico**: 10 cuerpos celestes con órbitas realistas
- **Sistema de Shaders Avanzados**: 11 tipos de shaders diferentes incluyendo:
  - Estrella: Animación de corona solar, erupciones y manchas solares dinámicas

  - Planetas rocosos: Texturas de terreno procedural con variaciones de altitud

  - Gigantes gaseosos: Bandas atmosféricas dinámicas en movimiento constante

  - Mundo de lava: Flujos de lava animados con pulsos de brillo intenso

  - Mundo helado: Superficies congeladas resplandecientes con acumulación de nieve

  - Planeta nuboso: Patrones de nubes animados sobre océanos y continentes

  - Planeta metálico: Superficie reflectante con patrones industriales

  - Planeta oceánico: Océanos con simulación de tormentas y patrones de olas

  - Planeta desértico: Dunas de arena y tormentas de polvo animadas

  - Gigante rayado: Bandas atmosféricas vibrantes con patrones de tormenta

 - Nave espacial: Material metálico con acentos cibernéticos y reflejos dinámicos

- **Ruido Procedural Avanzado**: Múltiples tipos de ruido (Perlin, Simplex, Voronoi) para generación de texturas
- **Sistema de Iluminación**: Iluminación global basada en world space
- **Cámara Interactiva**: Control de vista en 3D alrededor del sistema solar
- **Skybox Dinámico**: Fondo espacial con estrellas


### Cómo Compilar y Ejecutar

```bash
# Compilar en modo release
cargo build --release

# Ejecutar la aplicación
cargo run --release
```


### Controles

- WASD - Rotación de cámara              
- Q/E  - Desplazamiento horizontal       
- R/F  - Desplazamiento vertical         
- ↑/↓  - Zoom | Z/X - Avanzar/retroceder 
- O    - Mostrar/ocultar órbitas         
- V    - Mostrar/ocultar nave            
- ESC  - Salir                           

## Video demostrativo

[![Game running](https://img.youtube.com/vi/KpaX5xlLnqM/0.jpg)](https://www.youtube.com/watch?v=KpaX5xlLnqM)
