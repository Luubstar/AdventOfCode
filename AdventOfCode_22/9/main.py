import os

Casillas = []

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'


class Casilla:
    pos = (0,0)
    PasoCola = False
    EsInicio = False
    def __init__(self,x,y,Inic) -> None:
        self.pos = (x,y)
        self.EsInicio = Inic
    def ColaEstuvo(self):
        self.PasoCola(True)
    
    def FindCasilla(x,y):
        pos = (x,y)
        for i in Casillas:
            for j in i:
                if j.pos == pos:
                    return j

class Cola:
    apunta = None
    anteriorPosicion = (0,0)
    lastdelta = (0,0)
    pos = (0,0)    
    car = ""
    num = 0
    def __init__(self,x,y,car, apt,) -> None:
        self.pos = (x,y)
        self.car = car
        self.num = int(car)
        self.apunta = apt
    def mover(self):
        self.movDiagonal = False
        objetivo = self.apunta.pos
        delta = (objetivo[0] - self.pos[0],objetivo[1] - self.pos[1])
        
        if delta[0] != 0 and delta[1] != 0:
            self.pos = (self.pos[0] + delta[0]//abs(delta[0]), self.pos[1]+delta[1]//abs(delta[1]))
                
        elif delta[1] == 0 and delta[0] != 0:
            self.pos = (self.pos[0] + delta[0]//abs(delta[0]), self.pos[1])
        elif delta[0] == 0 and delta[1] != 0:
            self.pos = (self.pos[0] , self.pos[1]+ delta[1]//abs(delta[1]))
        
        self.lastdelta = delta
        casilla = Casilla.FindCasilla(self.pos[0],self.pos[1])
        if self.num == 9:
            casilla.PasoCola = True

    def CheckIfColisiona(self,x,y):
        return self.pos == (x,y) or self.pos == (x+1,y) or self.pos == (x,y+1) or self.pos == (x-1,y) or self.pos == (x,y-1) or self.pos == (x+1,y+ 1) or self.pos == (x-1,y+1) or self.pos == (x+1,y-1) or self.pos == (x-1,y-1)

class Cabeza:
    pos = (0,0)
    anteriorPosicion = (0,0)
    Colas =[]

    def __init__(self,x,y) -> None:
        self.pos = (x,y)
        self.anteriorPosicion=self.pos
        for i in range(3):
            if i == 0:
                self.Colas.append( Cola(x,y,i+1,self))
            else:
                self.Colas.append( Cola(x,y,i+1,self.Colas[i-1]))

    def CoincideConCola(self):
        self.Coince = self.pos == self.cola.pos
    
    def mover(self,x,y):
        self.anteriorPosicion = (self.pos[0],self.pos[1])
        self.pos = (x,y)
        for cola in self.Colas:
            if not cola.CheckIfColisiona(cola.apunta.pos[0],cola.apunta.pos[1]):
                cola.mover()
        self.printAll()
    
    def printAll(self):
        printed = []
        for casillaRow in Casillas:
            for casilla in casillaRow:
                if casilla.pos == self.pos and not printed.__contains__(self.pos):
                    print(f"{bcolors.OKGREEN}H{bcolors.ENDC}", end="")
                    printed.append(self.pos)
                for cola in self.Colas :
                    if casilla.pos == cola.pos and not printed.__contains__(cola.pos):
                        print(f"{bcolors.OKCYAN}{cola.car}{bcolors.ENDC}",end="")
                        printed.append(cola.pos)
                        break
                if not printed.__contains__(casilla.pos):
                    print("*",end="")
            print("")
            
        print("-------------\n\n\n")
        printed.clear()

with open(os.getcwd() + "/input") as f:
    input = f.read().split("\n")
    #input.remove(input[input.__len__() -1])

    xstart = 0
    ystart = 15
    xt = 16
    yt = 16
    for i in range(xt):
        newArray = []
        for j in range(yt):
            Inic = False
            if i == ystart and j == xstart:
                Inic = True
            newArray.append(Casilla(j,i, Inic))
        Casillas.append(newArray)

    CabezaMovible = Cabeza(xt/2,yt/2)
    orden = 0
    for linea in input:
            print(linea)
            if linea.startswith("R"):
                valor = int(linea.replace("R","").strip())
                paso = 1
                while (paso <= valor):
                    CabezaMovible.mover(CabezaMovible.pos[0] + 1, CabezaMovible.pos[1])
                    paso += 1
            elif linea.startswith("D"):
                valor = int(linea.replace("D","").strip())

                paso = 1
                while (paso <= valor):
                    CabezaMovible.mover(CabezaMovible.pos[0], CabezaMovible.pos[1] + 1)
                    paso += 1
            elif linea.startswith("L"):
                valor = int(linea.replace("L","").strip())

                paso = 1
                while (paso <= valor):
                    CabezaMovible.mover(CabezaMovible.pos[0] - 1, CabezaMovible.pos[1])
                    paso += 1
            elif linea.startswith("U"):
                valor = int(linea.replace("U","").strip())

                paso = 1
                while (paso <= valor):
                    CabezaMovible.mover(CabezaMovible.pos[0], CabezaMovible.pos[1] - 1)
                    paso += 1

cuenta = 0
for casillaRow in Casillas:
    for casilla in casillaRow:
        if casilla.PasoCola or casilla.EsInicio:
            print("#", end="")
            cuenta += 1
        else:
            print("*", end="")
    print("")

print(cuenta)