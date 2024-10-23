import os
final = 0
with open(os.getcwd() + "/input") as f:
    input = f.read()
    rondas = input.split("\n")
    rondas.remove("")
    for ronda in rondas:
        accion = ronda.split(" ")
        accionRival = accion[0]
        accionTuya = accion[1]
        

        if accionRival == "A":
            if accionTuya == "X":
                final += 0
                final += 3
            elif accionTuya == "Y":
                final += 3
                final += 1
            elif accionTuya == "Z":
                final += 6
                final += 2
        elif accionRival == "B":
            if accionTuya == "X":
                final += 0
                final += 1
            elif accionTuya == "Y":
                final += 3
                final += 2
            elif accionTuya == "Z":
                final += 6
                final += 3
        elif accionRival == "C":
            if accionTuya == "X":
                final += 0
                final += 2
            elif accionTuya == "Y":
                final += 3
                final += 3
            elif accionTuya == "Z":
                final += 6
                final += 1
    
print(final)