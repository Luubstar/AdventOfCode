import os
ListaCajas = [[], ["[S]","[M]","[R]","[N]","[W]","[J]","[V]","[T]"],["[B]","[W]","[D]","[J]","[Q]","[P]","[C]","[V]"],["[B]","[J]","[F]","[H]","[D]","[R]","[P]"],
 ["[F]","[R]","[P]","[B]","[M]","[N]","[D]"],["[H]","[V]","[R]","[P]","[T]","[B]"], ["[C]","[B]","[P]","[T]"], ["[B]","[J]","[R]","[P]","[L]"],
 ["[N]","[C]","[S]","[L]","[T]","[Z]","[B]","[W]"], ["[L]","[S]","[G]"]]

def printcajas():
    i = 0
    for item in ListaCajas:
        cajas = ""
        if i == 0:
            pass
        else:
            for caja in item:
                cajas += caja + " "
            print("fila nº", i, ":", cajas)
        i+=1

with open(os.getcwd() + "/input") as f:
    input = f.read().replace("move ", "").replace(" from ",",").replace(" to ", ",").split("\n")
    input.remove(input[input.__len__() -1])

    for entrada in input:

        datos = entrada.split(",")
        caja = ""
        cajasAux = []

        print("------------")
        for i in range(int(datos[0])):

            origen = int(datos[1])
            objetivo = int(datos[2])

            caja = ListaCajas[origen][ListaCajas[origen].__len__() -1]

            del ListaCajas[origen][-1]
            cajasAux.append(caja)
        
        for i in range(int(datos[0])):
            ListaCajas[objetivo].append(cajasAux[cajasAux.__len__() -1 -i])
            
        printcajas()
