# Proyecto de Arquitectura en Kubernetes para Aplicación gRPC con Escalado Automático y Monitoreo


## Descripción del Proyecto

Este proyecto implementa una arquitectura basada en contenedores y microservicios en Kubernetes para una aplicación gRPC. La aplicación incluye servicios para diferentes facultades y disciplinas, configurados con escalado automático y monitoreo en tiempo real.

## Requisitos Previos

Antes de iniciar, asegúrate de cumplir con los siguientes requisitos:

- Clúster de Kubernetes configurado en Google Kubernetes Engine (GKE).
- Herramientas instaladas:
  - `kubectl`
  - `helm`
  - `locust`
  - `Python 3` para la generación de tráfico con Locust.

## Arquitectura del Sistema


- **Facultades**: Los servicios `Ingeniería` y `Agronomía`, implementados en Rust y Go respectivamente, reciben solicitudes de los clientes y los redirigen a las disciplinas correspondientes.
- **Disciplinas**: Los servicios de `Natación`, `Atletismo`, y `Boxeo` determinan si el alumno es ganador o perdedor.
- **Consumidores**: LOs servidores de `Ganadores` y `Perdedores` son los encargados de recibir los datos de Kafka y guardarlo de manera ordenada den Redis
- **Kafka y Redis**: Configurados para la gestión de mensajes y almacenamiento de datos de resultados.
- **Monitoreo**: Configurado con Prometheus y Grafana para visualización en tiempo real.

## Despliegue de Servicios en Kubernetes

### Servicios de Facultad

Los servicios de facultad ([grcp-rust](./fac-Ingenieria/)
 y [grcp-app](./fac-Agronomia/)) están implementados en Rust y Go. Estos de aseguran de recibir en el client las solicitudes Http, para luego pasarlo al server por medio de gRCP y dirigirlo a las disciplinas

### Servicios de Disciplinas

Se encargan de recibir por medio de gRCP a los alumnos de su disciplina, determinar si es ganador o perdedor para luego ser enviado por kafka 

### Configuración de Ingress

El `Ingress` en este proyecto enruta el tráfico HTTP hacia dos servicios, `grpc-rust` y `grpc-app`, según la ruta de acceso:

- **Ruta `/ingenieria`**: Redirige el tráfico a `grpc-rust`, el servicio para la facultad de Ingeniería, en el puerto `8080`.
- **Ruta `/agronomia`**: Redirige el tráfico a `grpc-app`, el servicio para la facultad de Agronomía, en el puerto `8080`.

### Instalación del NGINX Ingress Controller

Para gestionar este enrutamiento, use el NGINX Ingress Controller, que expone el `Ingress` para redirigir el tráfico.

```bash
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml
```

Este comando descarga e instala el controlador de `Ingress` NGINX, que se encargará de manejar las solicitudes de entrada en Kubernetes y redirigirlas al servicio correspondiente basado en las rutas (`/ingenieria` y `/agronomia`).

### Configuración de Autoscaling (HPA)

Los HPAs están configurados para los servicios grpc-rust, grpc-app, disciplina-natacion, disciplina-atletismo, y disciplina-boxeo. Esto permite que el sistema se adapte dinámicamente al tráfico generado.
#### Ejemplo de HPA para `grpc-app`

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: grpc-app-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: grpc-app
  minReplicas: 1
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 20
```

## Generación de Tráfico con Locust

**Locust** genera tráfico de prueba para evaluar el rendimiento y escalabilidad de los servicios en Kubernetes. Simula solicitudes de alumnos a los endpoints del `Ingress` (`/ingenieria` y `/agronomia`), enviando datos en formato JSON para probar cómo los servicios `grpc-rust` y `grpc-app` manejan el tráfico. Esto permite observar cómo Kubernetes ajusta las réplicas de los pods mediante el `HPA` bajo condiciones de carga realista.

### Comandos de Instalación y Configuración de Locust

1. **Instalar Locust**

   ```bash
   
   pip install locust
   ```

2. **Ejecutar Locust**:

   Desde la carpeta con el archivo `app.py`, inicia Locust para enviar tráfico al `Ingress`:

   ```bash
   locust -f locustfile.py
   ```

3. **Configurar y Ejecutar la Prueba de Carga**:

   Abre `http://localhost:8089`, ajusta el número de usuarios simulados y la tasa de generación de tráfico

## Monitoreo y Visualización


### Grafana

**Grafana** es la herramienta de visualización que utilizamos para monitorear el rendimiento en tiempo real de los servicios en Kubernetes, mostrando métricas recolectadas por **Prometheus**.

**Instalación y Configuración**

1. **Instalar Grafana usando Helm**:

   ```bash
   helm repo add grafana https://grafana.github.io/helm-charts

   helm install grafana grafana/grafana --set adminPassword='<password>'
   ```

   Este comando instala Grafana en el clúster, configurando el panel de administración.

2. **Acceso a la Interfaz de Grafana**:

   Para acceder a Grafana en el navegador:

   ```bash
   kubectl port-forward svc/grafana 3000:80
   ```

   Una vez hecho esto, abre `http://localhost:3000` y usa las credenciales configuradas para iniciar sesión. Desde aquí, podrás configurar dashboards personalizados para visualizar el uso de CPU, tráfico HTTP y otras métricas de los servicios monitoreados.

---

### Redis

**Redis** almacena temporalmente los datos de ganadores y perdedores enviados por los servicios de disciplinas. Esto permite una recuperación rápida de los datos sin necesidad de acceso a una base de datos más pesada.

**Instalación**

1. **Instalar Redis usando Helm**:

   ```bash
   helm repo add bitnami https://charts.bitnami.com/bitnami

   helm install redis bitnami/redis --set auth.password=<password>
   ```

   Redis se instala con autenticación habilitada. Recuerda guardar la contraseña para futuras conexiones desde los servicios de disciplinas.

---

### Prometheus

**Prometheus** recolecta métricas de los servicios en Kubernetes, como el uso de CPU y tráfico HTTP, permitiendo que **Grafana** las visualice en tiempo real. Esta integración permite monitorear la escalabilidad automática (HPA) y el rendimiento general del sistema.

**Instalación**

1. **Instalar Prometheus usando Helm**:

   ```bash
   helm repo add prometheus-community https://prometheus-community.github.io/helm-charts

   helm repo update

   helm install prometheus prometheus-community/prometheus
   ```

---

## Configuración de Kafka con Strimzi

Kafka permite la comunicación entre los servicios de disciplinas y Redis para la gestión de mensajes de ganadores y perdedores.

### Instalación de Kafka y Strimzi

1. **Instalar el operador de Strimzi**:

   ```bash
   kubectl create -f 'https://strimzi.io/install/latest?namespace=default' -n default
   ```

2. **Desplegar Kafka en modo persistente**:

   ```bash
   kubectl apply -f https://strimzi.io/examples/latest/kafka/kafka-persistent-single.yaml -n default
   ```

3. **Verificar servicios de Kafka**:

   ```bash
   kubectl get svc -n default
   ```

4. **Instalar librería de Kafka en Go** (si es necesario para los servicios en Go):

   ```bash
   go get github.com/confluentinc/confluent-kafka-go/kafka
   ```

Con esta configuración, Kafka administra los mensajes para procesar ganadores y perdedores en los servicios de disciplinas, Redis almacena temporalmente los datos y Prometheus/Grafana ofrecen monitoreo y visualización en tiempo real del sistema en Kubernetes.

