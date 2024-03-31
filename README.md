<h1 align="center">arkwork-rs library examples</h1>

## Overview

En este respositorio se encuentran diferentes ejemplos de circuitos como puede ser el caso de Hadamard. En ellos se ve todo el proceso para construir circuitos zk-SNARKs con [arkwork-rs libraries](https://github.com/arkworks-rs) realizando Marlin.

En concreto, se pueden llamar a 3 versiones de Marlin. La primera de ellas es la versión original que esta definida ..., despué stenemos una versión más eficiente que es eliminando la parte del inner que se obtiene en ... y por último una asociada a la anterior que se obtiene en ...

## Build

```sh
cargo build --release --features print-trace
```

Para ejecutar se utiliza "--release" para conseguir que el ejecutable sea muchos más eficiente y "--features print-trace" para que se imprima por pantalla todo el proceso que se esta realizando.

## Run

```sh
./target/release/marlin --version version --circuit "circuit" --constraints constraint
```

Para entender mejor cuales son los parámetros que se deben introducir al ejecutable, a continaución se explica los posibles valores y para que sirven:

* "version": Dependiendo de la versión que se especifique (0, 1 o 2) se ejecutará una versión de Marlin u otra. Las diferencias son las que se han comentado anteriormente. Por defecto, se ejecuta la versión original, la 0.

* "circuit": En este proyecto se ofrecen ... circuitos. De modo que se deberá especificar cual es el que se quiere ejecutar, para ello hay que escribir en minúsculas el nombre del circuito tal y como esta en el fichero .rs sin "_circuit". Por defecto, se ejecuta el circuito de Hadamard.

* "constraint": Este parámetro sirve para determinar la grandaría del circuito. Para ello se debe poner un valor mayor a 0. Mencionar que cuánto mayor sea el valor mejores resultados nos dará, pero también tardará más en realizarse la prueba. Por defecto, toma el valor de 1 (dentro de cada circuito depende de sus características puede que este valor sea mayor).