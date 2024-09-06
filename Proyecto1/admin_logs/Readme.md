# Contenedor Administrador de Logs (Python)

## Descripción
Este componente es un servidor HTTP desarrollado con **FastAPI** que actúa como contenedor administrador de logs. Recibe logs enviados desde el servicio en Rust, los almacena en un archivo JSON y permite generar gráficas basadas en los datos registrados.

## Instalación

### 1. Requisitos Previos
- **Python 3.9** o superior
- **Docker**
- Bibliotecas necesarias listadas en `requirements.txt`:


### 2. Construcción del Contenedor Docker
Para construir el contenedor que ejecutará el servidor FastAPI:

1. Navega al directorio donde se encuentran los archivos `Dockerfile` y `requirements.txt`.
2. Ejecuta el siguiente comando para construir la imagen Docker:
 ```bash
 docker build -t admin_log .
```
### 3. Ejecutar el contenedor por separado
 ```bash
 docker run -d -p 8000:8000 -v $(pwd)/logs:/logs admin_log
```

## Uso
### 1. Recibir Logs
El servidor FastAPI está configurado para recibir logs a través de la ruta /logs. Los logs se envían en formato JSON y se almacenan en un archivo logs.json dentro del directorio montado.
Ejemplo de Petición

Para enviar logs, el servicio en Rust realiza una petición HTTP POST a la siguiente URL:
```
POST http://localhost:8000/logs
```
Cada log debe tener el siguiente formato JSON:

``` json 
{
  "pid": 1234,
  "name": "my_container",
  "cmd_line": "id_contenedor",
  "vsz": 102400,
  "rss": 51200,
  "memory_usage": 2.5,
  "cpu_usage": 15.0
}
```
### 2. Recibir Logs de RAM

El servidor también acepta logs de memoria RAM a través de la ruta /ram.
Ejemplo de Petición
```bash
POST http://localhost:8000/ram
```
El formato JSON esperado es:

```json
{
  "total_ram": 8192,
  "free_ram": 4096,
  "usage_ram": 4096
}
```

### 3. Generar Gráficas

El servidor puede generar dos tipos de gráficas:

- Uso de RAM en el tiempo
- Número de contenedores de alto y bajo consumo

Para generar las gráficas, el servidor expone la ruta /graficar. Al acceder a esta ruta, el servidor leerá los logs y generará las gráficas en formato PNG dentro del directorio logs.
Ejemplo de Petición

```bash
GET http://localhost:8000/graficar
```
