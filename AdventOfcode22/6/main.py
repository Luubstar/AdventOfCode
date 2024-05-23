import os
letras = []
pasos = 0
with open(os.getcwd() + "/input") as f:
    input = list(f.read())
    input.remove(input[input.__len__() - 1])
    for i in range(input.__len__()):
        canEnd = False
        if letras.__len__() < 14:
            letras.append(input[i])
        elif letras.__len__() == 14:
            del letras[0]
            letras.append(input[i])
            canEnd = True
            for letra in letras:
                if letras.count(letra) > 1:
                    canEnd = False
                    break
        if canEnd:
            pasos = i + 1
            break
print(pasos,letras)
