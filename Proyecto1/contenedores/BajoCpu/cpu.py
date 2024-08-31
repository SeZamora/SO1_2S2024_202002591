import time

def consume_low_cpu():
    while True:
        time.sleep(10)  # Duerme para reducir el consumo de CPU

if __name__ == "__main__":
    consume_low_cpu()
