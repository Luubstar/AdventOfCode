Pares = []
ParesOrdenados = []
with open("input") as f:
    input = f.read().split("\n\n")
    for linea in input:
        Pares.append(linea.split("\n"))

buenorden = 0
log = ""
def operalistas(lista1, lista2):
    global log
    contador = min(len(lista1), len(lista2))
    estabien = False
    for i in range(contador):
        if isinstance(lista1[i],int) and isinstance(lista2[i],int):
            if lista1[i] < lista2[i]:
                log += "\n " + str(lista1[i]) + " es menor que " + str(lista2[i]) + " por tanto True"
                return True
            elif lista2[i] < lista1[i]:
                log += "\n " + str(lista1[i]) + " es mayor que " + str(lista2[i]) 
                return False
            elif lista1 == lista2:
                log += "\n Son iguales " + str(lista1[i])+ " y " + str(lista2[i])
                return None
        else:
            if isinstance(lista1[i], int):
                log += "\n " + str(lista1[i]) + " convertido en lista"
                lista1[i] = [lista1[i]] 
            elif isinstance(lista2[i], int):
                log += "\n " + str(lista2[i]) + " convertido en lista"
                lista2[i] = [lista2[i]] 

            log += "\n " + str(lista1[i]) + " y " + str(lista2[i]) + " son listas. Se operan"
            estabien = operalistas(lista1[i], lista2[i])
            if type(estabien) == bool:
                log += "\n Se ha devuelto " + str(estabien)
                return estabien

    if len(lista1) < len(lista2):
        log += "\n Todo es igual menos el tamaño de la lista. Está bien ordenado porque "+ str(lista1) + " es más corta que " + str(lista2)
        return True
    elif len(lista2) < len(lista1):
        log += "\n Todo es igual menos el tamaño de la lista. Está mal ordenado porque "+ str(lista1) + " es más larga que " + str(lista2)
        return False
    elif len(lista1) == len(lista2):
        return None
    log += "\n Algo ha salido mal"
    return False



for pareja in Pares:
    lista1 = eval(pareja[0])
    lista2 = eval(pareja[1])
    ParesOrdenados.append(lista1)
    ParesOrdenados.append(lista2)
    
Continuar = True

ParesOrdenados.append([6])
ParesOrdenados.append([2])

while (Continuar):
    Continuar = False
    for i in range(0, len(ParesOrdenados) -1 ): 
        listaUno = ParesOrdenados[i]
        listaDos = ParesOrdenados[i +1]
        if not operalistas(listaUno, listaDos):
            ParesOrdenados[i], ParesOrdenados[i+1]
            Continuar = True
    


print("\n" + str(ParesOrdenados))

print((ParesOrdenados.index([[2]]) +1) * (ParesOrdenados.index([[6]])+1))
