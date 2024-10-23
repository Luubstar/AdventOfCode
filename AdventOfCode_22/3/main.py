import os
mochilas = []
grupo3 = []
comunes = ""
res = 0
prioridades = ["-","a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"]
with open(os.getcwd() + "/input") as f:
    input = f.read()
    mochilas = input.split("\n")
    mochilas.remove(mochilas[mochilas.__len__() -1])
    for mochila in mochilas:
        grupo3.append(mochila)
        if grupo3.__len__() == 3:
            charRack = []
            for linea in grupo3:
                charArray = [char for char in linea]
                for char in charArray: charRack.append(char)
            for char in charRack:
                if grupo3[0].__contains__(char) and grupo3[1].__contains__(char) and grupo3[2].__contains__(char):
                    comunes += char
                    break
            grupo3.clear()
print(comunes,comunes.__len__())
for letra in comunes:
    valor = 0
    if letra.isupper(): 
        valor += 26
    valor += prioridades.index(letra.lower())
    res += valor
print(res)