import os
ArrayArboles = []
ancho = 0
largo = 0
visible = 0
with open(os.getcwd() + "/input") as f:
    input = f.read().split("\n")
    input.remove(input[input.__len__() -1])
    for linea in input:
        lista = list(linea)
        newlist = []
        for arbol in lista:
            newlist.append(arbol)
        ArrayArboles.append(newlist)


y = 0
x = 0
ListaCuenta = []
for linea in ArrayArboles:
    if y > 0 and y < ArrayArboles.__len__()-1:
        x = 1
        while(x < linea.__len__()-1):
            ListaCuenta.clear()
            arbolcheck = ArrayArboles[y][x]
            tempy = y -1
            up = ArrayArboles[y-1][x]
            cuenta = 1

            while(tempy >= 0 and arbolcheck > ArrayArboles[tempy][x]):
                cuenta += 1
                tempy -= 1
            if (cuenta > y):
                cuenta -= 1

            ListaCuenta.append(cuenta)
            tempy = y + 1
            down = ArrayArboles[y+1][x]
            cuenta = 1
            while(tempy < ArrayArboles.__len__() and arbolcheck > ArrayArboles[tempy][x]):
                cuenta += 1
                tempy += 1
            if (cuenta + y >  ArrayArboles.__len__() - 1):
                cuenta -= 1

            ListaCuenta.append(cuenta)
            left = ArrayArboles[y][x-1]
            tempx = x - 1
            cuenta = 1

            while(tempx >= 0 and arbolcheck > ArrayArboles[y][tempx]):
                cuenta += 1
                tempx -= 1
            if (cuenta > x):
                cuenta -= 1

            ListaCuenta.append(cuenta)
            rigth = ArrayArboles[y][x+1]
            tempx = x + 1
            cuenta = 1

            while(tempx < linea.__len__() and arbolcheck > ArrayArboles[y][tempx]):
                cuenta += 1
                tempx += 1
            if (cuenta + x >  linea.__len__() - 1):
                cuenta -= 1

            ListaCuenta.append(cuenta) 
            valor = 1
            for cuentas in ListaCuenta:
                valor *= cuentas
            if visible < valor:
                visible = valor
            x+= 1
    y += 1

print(visible)