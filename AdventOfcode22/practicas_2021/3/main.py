import os
entradas = []
entradas_aux =[]
oxigeno = ""
co2 = ""
with open(os.getcwd() + "/input") as f:
    inputs = f.read().split("\n")
    for dato in inputs:
        if dato != " ":
            entradas.append(dato)
    i = 0
    entradas.remove(entradas[entradas.__len__()-1])


    while i < entradas[0].__len__() + 1:
        if entradas.__len__() > 1:
            count0 = 0
            count1 = 0
            for dato in entradas:
                if dato[i] == "1":
                    count1 +=1
                else:
                    count0 += 1
            if count1 >= count0:
                for dato in entradas:
                    if dato[i] == "1":
                        entradas_aux.append(dato)
            elif count0 > count1:
                for dato in entradas:
                    if dato[i] == "0":
                        entradas_aux.append(dato)
            entradas.clear()
            for dato in entradas_aux:
                entradas.append(dato)
            entradas_aux.clear()
        else:
            oxigeno = entradas[0]
        i+=1

    entradas.clear()
    for dato in inputs:
        if dato != " ":
            entradas.append(dato)
    i = 0
    entradas.remove(entradas[entradas.__len__()-1])

    
    i = 0
    while i < entradas[0].__len__() + 1:
        if entradas.__len__() > 1:
            count0 = 0
            count1 = 0
            for dato in entradas:
                if dato[i] == "1":
                    count1 +=1
                else:
                    count0 += 1
            if count1 < count0:
                for dato in entradas:
                    if dato[i] == "1":
                        entradas_aux.append(dato)
            elif count0 <= count1:
                for dato in entradas:
                    if dato[i] == "0":
                        entradas_aux.append(dato)
            entradas.clear()
            for dato in entradas_aux:
                entradas.append(dato)
            entradas_aux.clear()
        else:
            co2 = entradas[0]
        i+=1

        
print(oxigeno, co2)
print(int(oxigeno,2) * int(co2,2))