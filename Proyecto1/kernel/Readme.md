# Módulo de Kernel(C)

## Descripción
Este módulo de kernel en **C** tiene como objetivo capturar las métricas de uso de memoria y CPU de los procesos que se están ejecutando en el sistema, especialmente aquellos asociados con contenedores Docker. La información se guarda en un archivo dentro del sistema de archivos `/proc` llamado `sysinfo_202002591`. Esta información es consumida por el servicio en **Rust** para la gestión de contenedores.

## Instalación

### 1. Requisitos Previos
- **Linux Kernel** con soporte para módulos
- **Herramientas de desarrollo del kernel** (incluyendo `make`, `gcc`, y los headers del kernel)

### 2. Compilación del Módulo
Ejecuta el siguiente comando para compilar el módulo:
   ```bash
   make
   ```
### 3. Cargar el Módulo en el Kernel

Para cargar el módulo y que empiece a capturar las métricas:

```bash
sudo insmod sysinfo_202002591.ko
```
### 4. Desactivar el Módulo

Para remover el módulo del kernel:

```bash
sudo rmmod sysinfo_202002591
```

## Modo de Uso
### 1. Lectura de Métricas

Después de cargar el módulo, puedes leer las métricas del sistema ejecutando:

```bash
cat /proc/sysinfo_202002591
```
El archivo mostrará información en formato JSON, que incluye el uso de memoria y CPU de los procesos asociados con Docker. Ejemplo de salida:

```json
{
  "Total RAM": 8192,
  "Free RAM": 4096,
  "RAM Uso": 4096,
  "Procesos": [
    {
      "PID": 1234,
      "Nombre": "python",
      "Linea de Comando": "container123",
      "Vsz": 102400,
      "Rss": 51200,
      "Memoria Usada": 2.50,
      "Cpu Usado": 15.00
    }
  ]
}
```
