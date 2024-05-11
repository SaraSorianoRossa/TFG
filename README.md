## Descripción general

En este respositorio se encuentran diferentes ejemplos de circuitos como puede ser el caso del Producto de Hadamard. En ellos se ve todo el proceso para construir circuitos zk-SNARKs con [arkwork-rs libraries](https://github.com/arkworks-rs) realizando Marlin.

En concreto, se pueden llamar a 5 versiones de Marlin. La primera de ellas es la [versión original](https://github.com/SaraSorianoRossa/Original-Marlin) con algunas adaptaciones en el código para poder ser utilizado en este trabajo. A continuación, se realizó la [primera modificación](https://github.com/SaraSorianoRossa/Marlin-v2) cuya versión es más eficiente ya que se elimina la parte del inner (proceso que puede realizar cualquier persona). Una vez realizada la anterior modificación se vió que el envío de $t(X)$ no era necesario. Por ello, se realizó una [segunda modificación](https://github.com/SaraSorianoRossa/Marlin-v3). La [tercera modificación](https://github.com/SaraSorianoRossa/Marlin-v4) consiste en eliminar el polinomio $s(X)$. Este hecho provoca que el circuito no tenga conocimiento nulo por completo consiguiendo una eficiencia mayor. El conocimiento nulo se intentará corregir en trabajos futuros, pero conseguirlo no provocará que sea menos eficiente. Por último, tal y como se explica en la memoria, se generó un nuevo proceso para verificar que la abertura del polinomio $t(X)$ es correcta, consiguiendo demostrar al verificador que toda la información calculada es verídica. Esta [última modificación](https://github.com/SaraSorianoRossa/Marlin-v5) es la considerada versión 5.

## Ejecutar

```sh
cargo build --release --features print-trace
```

Para ejecutar se utiliza ``--release`` para conseguir que el ejecutable sea muchos más eficiente y ``--features print-trace`` para que se imprima por pantalla todo el proceso que se esta realizando.

## Run

```sh
./target/release/marlin --version version --circuit "circuit" --constraints constraint --groth16 "isgroth16"
```

Para entender mejor cuales son los parámetros que se deben introducir al ejecutable, a continaución se explica los posibles valores y para que sirven:

* ```version```: Dependiendo de la versión que se especifique (1, 2, 3, 4 o 5) se ejecutará una versión de Marlin u otra, la versión 5 hace referencia a aplicar y calcular el tiempo del nuevo proceso inner. Las diferencias son las que se han comentado anteriormente. Por defecto (si no se especifica el parámetro), se ejecuta la versión original, la 1.

* ```"circuit"```: En este proyecto se ofrecen 3 circuitos. De modo que se deberá especificar cual es el que se quiere ejecutar, para ello hay que escribir en minúsculas el nombre del circuito tal y como esta en el fichero .rs sin "_circuit". Por defecto, se ejecuta el circuito de Hadamard. Si queremos ejecutar el de "addition_circuit" deberíamos escribir solamente "addition".

* ```constraint```: Este parámetro sirve para determinar la grandaría del circuito. Para ello se debe poner un valor mayor a 0. Mencionar que cuánto mayor sea el valor mejores resultados nos dará, pero también tardará más en realizarse la prueba (este tiempo a partir de realizar pruebas se ha observado que depende de las características del ordenador donde se ejecuta). Por defecto, toma el valor de 1 (dentro de cada circuito depende de sus características puede que este valor sea mayor).

* ```"isgroth16"```: Al añadir este parámetro se consigue que el usuario pueda decidir si quiere que el circuito también se ejecute con Groth16 ("true") o por el contrario solo desea que se ejecute en Marlin ("false").

Un ejemplo de ejecución es la siguiente:
```sh
./target/release/marlin --version 2 --circuit "hadamard" --constraints 5000 --groth16 "true"
```
