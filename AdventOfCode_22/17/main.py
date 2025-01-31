class Pieza:
    x = 0
    y = 0

    def __init__(self,y,x) -> None:
        self.x = x
        self.y = y

class Rock:
    Estacionado = False

    def __init__(self,piezas) -> None:
        self.Partes = piezas

    def CheckCollision(self):
        for parte in self.Partes:
            for roca in Rocas:
                for parteroca in roca:
                    if (parteroca.y == parte.y-1 and not parteroca == parte and not parteroca in self.Partes) or parte.y-1 < 0 :
                        return True

def printall():
    print("-------------")
    string = ""
    for i in range(Altura+5,-1,-1):
        for x in range(0,7):
            coordenadas = (x,i)
            if PosDict.get(coordenadas) != None:
                print("#",end="")
            else:
                print(".",end="")
        print("")
    print("-----------")


with open("input") as f:
    inputs = list(f.read())


CuentaRocas = 0
Roca = None
Rocas = []
PosDict = {}
Altura = 0

i = 0
rocas = 0
while(rocas < 2023):
    if i == len(inputs):
        i = 0
    
    if Roca == None:
        if CuentaRocas == 0:
            Pieza1 = Pieza(Altura + 3, 2)
            Pieza2 = Pieza(Altura + 3, 3)
            Pieza3 = Pieza(Altura + 3, 4)
            Pieza4 = Pieza(Altura + 3, 5)
            Roca = Rock([Pieza1,Pieza2, Pieza3, Pieza4])
        if CuentaRocas == 1:
            Pieza1 = Pieza(Altura + 3, 3)
            Pieza2 = Pieza(Altura + 4, 2)
            Pieza3 = Pieza(Altura + 4, 3)
            Pieza4 = Pieza(Altura + 4, 4)
            Pieza5 = Pieza(Altura + 5, 3)
            Roca = Rock([Pieza1,Pieza2, Pieza3, Pieza4, Pieza5])
        if CuentaRocas == 2:
            Pieza1 = Pieza(Altura + 3, 2)
            Pieza2 = Pieza(Altura + 3, 3)
            Pieza3 = Pieza(Altura + 3, 4)
            Pieza4 = Pieza(Altura + 4, 4)
            Pieza5 = Pieza(Altura + 5, 4)
            Roca = Rock([Pieza1,Pieza2, Pieza3, Pieza4, Pieza5])
        if CuentaRocas == 3:
            Pieza1 = Pieza(Altura + 3, 2)
            Pieza2 = Pieza(Altura + 4, 2)
            Pieza3 = Pieza(Altura + 5, 2)
            Pieza4 = Pieza(Altura + 6, 2)
            Roca = Rock([Pieza1,Pieza2, Pieza3, Pieza4])
        if CuentaRocas == 4:
            Pieza1 = Pieza(Altura + 3, 2)
            Pieza2 = Pieza(Altura + 3, 3)
            Pieza3 = Pieza(Altura + 4, 2)
            Pieza4 = Pieza(Altura + 4, 3)
            Roca = Rock([Pieza1,Pieza2, Pieza3, Pieza4 ])
        Rocas.append(Roca.Partes)
        for parte in Roca.Partes:
            coordenadas = (parte.x,parte.y)
            PosDict[coordenadas] = parte
        CuentaRocas += 1
        if CuentaRocas > 4:
            CuentaRocas = 0
        rocas += 1
    else:

            direccion = inputs[i]
            print(direccion)
            if direccion == "<":
                posx = []
                for pieza in Roca.Partes:
                    posx.append(pieza.x - 1)
                canmove = True
                for pos in posx:
                    if pos <0:
                        canmove = False
                if canmove:     
                    for pieza in Roca.Partes:
                        coordenadas = (parte.x, parte.y)
                        PosDict.pop(coordenadas, None)
                        coordenadas = (parte.x-1, parte.y)
                        PosDict[coordenadas] = parte
                        pieza.x -= 1
            else:
                posx = []
                for pieza in Roca.Partes:
                    posx.append(pieza.x + 1)
                canmove = True
                for pos in posx:
                    if pos >6:
                        canmove = False
                if canmove:     
                    for pieza in Roca.Partes:
                        coordenadas = (parte.x, parte.y)
                        PosDict.pop(coordenadas, None)
                        coordenadas = (parte.x+1, parte.y)
                        PosDict[coordenadas] = parte
                        pieza.x += 1
            
            if Roca.CheckCollision():
                for pieza in Roca.Partes:
                    Altura = max(pieza.y+1, Altura)
                Roca.Estacionado = True

            if not Roca.Estacionado:
                for parte in Roca.Partes:
                    coordenadas = (parte.x, parte.y)
                    PosDict.pop(coordenadas, None)
                    coordenadas = (parte.x, parte.y-1)
                    PosDict[coordenadas] = parte
                    parte.y -= 1
            else:
                Roca = None

            i += 1
print(Altura)

