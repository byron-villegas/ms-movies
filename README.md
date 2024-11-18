# Instalar paquetes
Se debe ejecutar el siguiente comando

```shell
cargo build
```

# Ejecutar aplicacion
Se debe ejecutar el siguiente comando

```shell
cargo run
```

# Tests Unitarios

### Ejecutarlo
Se debe ejecutar el siguiente comando

```shell
cargo test
```

### Instalar libreria para cobertura
Se debe ejecutar el siguiente comando

```shell
cargo install cargo-llvm-cov
```

### Ejecutarlo con cobertura
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs"
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos


### Generar reporte html de cobertura
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs" --html
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos