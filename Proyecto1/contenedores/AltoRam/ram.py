import time

def consume_ram():
    data = []
    for i in range(35):
        data.append([0] * 1000000)  
    while  True:
        time.sleep(10)  

if __name__ == "__main__":
    consume_ram()
