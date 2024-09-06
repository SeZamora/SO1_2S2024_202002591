from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import json
import os
import matplotlib.pyplot as plt
from datetime import datetime

app = FastAPI()
LOG_FILE = "./logs/logs.json" 

# Modelo para el proceso de logs
class LogProcess(BaseModel):
    pid: int
    name: str
    cmd_line: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float

class LogRam(BaseModel):
    total_ram: int
    free_ram: int
    usage_ram: int


# Ruta para recibir los logs
@app.post("/logs")
async def receive_logs(logs: list[LogProcess]):
    if os.path.exists(LOG_FILE):
        with open(LOG_FILE, "r") as f:
            data = json.load(f)
    else:
        data = []

    # Agregar nuevos logs con la fecha y hora actual
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    for log in logs:
        data.append({"timestamp": timestamp, **log.dict()})

    with open(LOG_FILE, "w") as f:
        json.dump(data, f, indent=4)

    return {"message": "Logs received successfully"}

@app.post("/ram")
async def receive_ram(ram: LogRam):
    if os.path.exists(LOG_FILE):
        with open(LOG_FILE, "r") as f:
            data = json.load(f)
    else:
        data = []

    # Agregar nuevos logs con la fecha y hora actual
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    data.append({"timestamp": timestamp, **ram.dict()})

    with open(LOG_FILE, "w") as f:
        json.dump(data, f, indent=4)

    return {"message": "Logs received successfully"}


# Ruta para generar gráficas
@app.get("/graficar")
async def graficar():
    if not os.path.exists(LOG_FILE):
        raise HTTPException(status_code=404, detail="Log file not found")

    with open(LOG_FILE, "r") as f:
        logs = json.load(f)

    # Filtrar logs relacionados con la RAM
    ram_logs = [log for log in logs if 'total_ram' in log]

    # Gráfica 1: Uso de RAM en el tiempo
    timestamps_ram = [datetime.strptime(log['timestamp'], "%Y-%m-%d %H:%M:%S") for log in ram_logs]
    usage_ram = [log['free_ram'] for log in ram_logs]

    plt.figure(figsize=(10, 5))
    plt.plot(timestamps_ram, usage_ram, label="RAM Libre (KB)", marker='o')
    plt.xlabel("Timestamp")
    plt.ylabel("RAM libre (KB)")
    plt.title("RAM Libre en el Tiempo")
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig("./logs/ram_free.png")  # Guarda la gráfica en un archivo
    plt.show()

    # Filtrar logs de procesos
    process_logs = [log for log in logs if 'pid' in log]

    # Contar los de alto y bajo consumo
    high_consumption_count = 0
    low_consumption_count = 0
    timestamps_containers = []
    high_consumption = []
    low_consumption = []

    for log in process_logs:
        timestamps_containers.append(datetime.strptime(log['timestamp'], "%Y-%m-%d %H:%M:%S"))
        if log['memory_usage'] >= 4.0 or log['cpu_usage'] >= 25.0:
            high_consumption_count += 1
            high_consumption.append(1)
            low_consumption.append(0)
        else:
            low_consumption_count += 1
            high_consumption.append(0)
            low_consumption.append(1)

    # Gráfica 2: Contenedores de alto y bajo consumo en el tiempo
    # Filtrar logs de procesos
    process_logs = [log for log in logs if 'pid' in log]

    # Inicializar contadores
    high_consumption_count = 0
    low_consumption_count = 0

    # Contar los de alto y bajo consumo
    for log in process_logs:
        if log['memory_usage'] >= 4.0 or log['cpu_usage'] >= 25.0:
            high_consumption_count += 1
        else:
            low_consumption_count += 1

    # Etiquetas y valores para la gráfica de barras
    categories = ['Alto Consumo', 'Bajo Consumo']
    values = [high_consumption_count, low_consumption_count]

    # Crear la gráfica de barras
    plt.figure(figsize=(7, 5))
    plt.bar(categories, values, color=['red', 'green'])
    plt.xlabel("Tipo de Contenedor")
    plt.ylabel("Numero de Contenedores")
    plt.title("Alto Consumo vs Bajo Consumo contenedores")
    plt.tight_layout()
    plt.savefig("./logs/contenedores_creados.png")  
    plt.show()

    return {"message": "Graphs generated successfully"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
