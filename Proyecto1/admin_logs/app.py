from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import json
import os
import matplotlib.pyplot as plt
from datetime import datetime

app = FastAPI()
LOG_FILE = "/app/logs/logs.json"  # Ruta dentro del contenedor donde se almacenan los logs

# Modelo para el proceso de logs
class LogProcess(BaseModel):
    pid: int
    name: str
    cmd_line: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float

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

# Ruta para generar gráficas
@app.get("/graficar")
async def graficar():
    if not os.path.exists(LOG_FILE):
        raise HTTPException(status_code=404, detail="Log file not found")

    with open(LOG_FILE, "r") as f:
        data = json.load(f)

    # Generar gráfica de uso de memoria
    timestamps = [entry["timestamp"] for entry in data]
    memory_usages = [entry["memory_usage"] for entry in data]

    plt.figure(figsize=(10, 5))
    plt.plot(timestamps, memory_usages, label="Memory Usage")
    plt.xlabel("Timestamp")
    plt.ylabel("Memory Usage")
    plt.title("Memory Usage Over Time")
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig("/app/logs/memory_usage.png")

    # Generar gráfica de uso de CPU
    cpu_usages = [entry["cpu_usage"] for entry in data]

    plt.figure(figsize=(10, 5))
    plt.plot(timestamps, cpu_usages, label="CPU Usage", color="red")
    plt.xlabel("Timestamp")
    plt.ylabel("CPU Usage")
    plt.title("CPU Usage Over Time")
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig("/app/logs/cpu_usage.png")

    return {"message": "Graphs generated successfully"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
