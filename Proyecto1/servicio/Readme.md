# Servicio de Gestión de Contenedores (Rust)

## Descripción
El servicio desarrollado en **Rust** es el núcleo del proyecto. Su principal responsabilidad es la gestión de contenedores Docker en el sistema, capturando métricas del uso de CPU y memoria de los procesos asociados y enviando logs al servidor en Python.


## Instalación

### 1. Requisitos Previos
- **Rust** (debe estar instalado en el sistema)
- **Docker** (para la gestión de contenedores)
- **Cargo**, el gestor de paquetes de Rust, para compilar el proyecto

### 2. Compilación del Proyecto

Compila el servicio en Rust utilizando Cargo:

```bash
cargo run --release
```

## Modo de Uso


### 1. Flujo del Servicio

- Lectura de Métricas: El servicio lee las métricas de uso de CPU y memoria desde un archivo ubicado en /proc/sysinfo_202002591. Este archivo es generado por un módulo de kernel escrito en C.

- Envío de Logs: Cada 10 segundos, el servicio analiza los datos de los contenedores y envía logs al servidor en Python. Estos logs contienen información sobre el uso de memoria y CPU de los contenedores.

- Gestión de Contenedores: Dependiendo de los resultados del análisis, el servicio decide qué contenedores eliminar. Las reglas generales son:
    - Siempre deben quedar 2 contenedores de alto consumo.
    - Siempre deben quedar 3 contenedores de bajo consumo.

- Los contenedores que no cumplan estos criterios serán eliminados.

### 2. Señal de Finalización

El servicio puede finalizar de forma controlada mediante la señal Ctrl + C. Al hacerlo:
- El servicio elimina el cronjob que crea contenedores.
- Se realiza una última petición al servidor de logs para generar gráficas.