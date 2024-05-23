import os
ElfosDatos = []
Elfoscal = []
listaCalorias = []
caloriasElfo = 0
with open(os.getcwd() + "/2º/Input/input") as f:
    input = f.read()
    ElfosDatos = input.split("\n\n")
    for elfo in ElfosDatos:
        Elfoscal = elfo.split("\n")
        caloriasElfo = 0
        for comida in Elfoscal:
            if comida != "":
                caloriasElfo += int(comida)
        listaCalorias.append(caloriasElfo)

listaCalorias.sort()
print(listaCalorias)
total = listaCalorias[listaCalorias.__len__() -1] + listaCalorias[listaCalorias.__len__() -2] + listaCalorias[listaCalorias.__len__() -3]
print(total)


    