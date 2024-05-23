import os
profundidades = []
Grupos = []
grupocuenta = 1
cuenta = 0
profundidad_anterior=0
with open(os.getcwd() + "/1/input") as f:
    profundidades = f.read().split("\n")
    Grupos.append(0)
    Grupos.append(0)
    for profundidad in profundidades:
        if profundidad != "":
            i = grupocuenta -3
            if i < 0:
                i = 0
            while(i < grupocuenta):
                Grupos[i] = Grupos[i] + int(profundidad)
                i+=1
            grupocuenta += 1
            Grupos.append(0)

prev_item = 440
for item in Grupos:
    if item != 0:
        if item > prev_item:
            cuenta += 1
        prev_item = item
            
print(Grupos)


print(cuenta)