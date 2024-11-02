from locust import HttpUser, task, between
import random

# Generar un conjunto grande de datos (10,000 objetos) para simular tráfico
data_pool = [
    {
        "nombre": f"Estudiante{i}",
        "edad": random.randint(18, 25),
        "facultad": random.choice(["Ingenieria", "Agronomia"]),
        "disciplina": random.choice([1, 2, 3])  # 1: Natación, 2: Atletismo, 3: Boxeo
    }
    for i in range(10000)
]

class WebsiteUser(HttpUser):
    wait_time = between(1, 3)  # Intervalo de espera entre solicitudes
    host = "http://34.123.102.164"

    @task
    def enviar_trafico(self):
        # Seleccionar un objeto de datos aleatorio
        payload = random.choice(data_pool)
        
        # Determinar la ruta según la facultad
        endpoint = "/ingenieria" if payload["facultad"] == "Ingenieria" else "/agronomia"
        
        # Enviar la solicitud POST
        self.client.post(endpoint, json=payload)
