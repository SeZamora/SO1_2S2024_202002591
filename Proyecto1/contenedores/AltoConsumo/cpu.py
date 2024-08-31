import time
import math

def moderate_cpu_load():

    while True:
        start_time = time.time()


        for _ in range(1000000):
            math.sqrt(12345)


        time.sleep(1.5) 


        elapsed_time = time.time() - start_time


        if elapsed_time < 0.1:
            time.sleep(0.1 - elapsed_time)

if __name__ == "__main__":
    moderate_cpu_load()
