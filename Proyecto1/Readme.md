# Gestor de Contenedores Docker con Captura de Métricas del Sistema

## Descripción General
Este proyecto consiste en un **gestor de contenedores Docker** que combina un **módulo de kernel en C**, un **servicio en Rust** y una **API en Python** para capturar métricas de uso de recursos del sistema y gestionar los contenedores en función de dichas métricas.


## Estructura del Repositorio

- **[admin_logs](./admin_logs/)**: Contiene la aplicación en **Python** que actúa como servidor de logs. Recibe y almacena los logs enviados por el servicio en Rust, y genera gráficas del uso de recursos.
  
- **[servicio](./servicio/)**: Contiene la aplicación en **Rust**, encargada de la gestión de contenedores. Lee las métricas del sistema capturadas por el módulo de kernel y toma decisiones sobre los contenedores.

- **[kernel](./kernel/)**: Contiene el código del **módulo de kernel en C**, que captura las métricas de memoria y CPU de los procesos en ejecución. La información se guarda en el archivo `/proc/sysinfo_202002591`.

- **[contenedores](./contenedores/)**: Esta carpeta contiene las imágenes Docker de contenedores de alto y bajo consumo. Estas imágenes se utilizan para simular diferentes niveles de carga en el sistema.

- **[contenedores.sh](./contenedores.sh/)**: Script de **bash** para automatizar la creación de contenedores Docker de alto y bajo consumo. El script genera contenedores aleatoriamente utilizando las imágenes almacenadas en la carpeta `contenedores`.

## Flujo Completo del Sistema

- ### Inicio del Proceso:
    - El script contenedores.sh con un cronjob crea contenedores aleatorios cada 30 segundos.
    - El módulo de kernel captura métricas de estos contenedores y otros procesos del sistema.

- ### Ejecución del Servicio:
    - Crea un contendor de la aplicacion que registra las todas las peticiones `http`
    - El servicio en Rust lee las métricas desde el archivo `/proc`.
    - El servicio analiza los contenedores y decide cuáles eliminar o mantener.
    - Envía logs al servidor en Python para registrar las decisiones.

- ### Finalizar el programa:
    - Al final de la ejecución, el servidor en Python genera gráficas con la informacion generada por los logs.
    - Elimina en cronjob
