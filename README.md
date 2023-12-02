# Advent of Code 2023

Este repositorio es para mostrar mi participación en esta serie de desafíos.

Estos desafíos se pueden encontrar [en la página de Advent of Code](https://adventofcode.com/2023).

## Cómo utilizar:

Estoy haciendo estos desafíos en Rust. El repositorio es un workspace donde cada día es un proyecto de cargo de tipo lib. La forma de probar mis respuestas es con la interfaz de pruebas de cargo:

```shell
# Ejemplo: Día 1.
cargo test -p day1 -- --no-capture
```

Es necesario colocar el `-- --nocapture` ya que estoy imprimiendo mis respuestas finales dentro de los test!
