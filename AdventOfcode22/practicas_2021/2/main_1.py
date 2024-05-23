import os
instrucciones = []
aim = 0
x = 0
profundidad = 0
with open(os.getcwd() + "/1/input") as f:
    instrucciones = f.read().split("\n")
    for codigo in instrucciones:
        if codigo != "":
            orden = codigo.split(" ")[0]
            cantidad = int(codigo.split(" ")[1])
            if orden == "forward":
                x += cantidad
                profundidad += aim * cantidad
            elif orden == "down":
                aim += cantidad
            elif orden == "up":
                aim -= cantidad

print(x*profundidad)