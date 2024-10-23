from PIL import Image
import numpy as np
import os
import threading
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


class Tile:
    x = 0
    y = 0
    PuntoD = None
    PuntoDI = None
    PuntoDR = None
    type = "Air"
    symbol = "."
    def __init__(self,x,y) -> None:
        self.x = x
        self.y = y
    def cambiarTipo(self, simbolo, tipo):
        self.type = tipo
        self.symbol = simbolo
        return self
    def move(self,TileObj):
        TileObj.cambiarTipo(self.symbol, self.type)
        self.symbol = "."
        self.type = "Air"
    def canLand(self, punto):
        if punto != None:
            return punto.type == "Air" 
        return False
    def Fall(self,Mapa):
        if self.y+1 < len(Mapa.Mapa):
            punto = self.PuntoD
            if self.canLand(punto):
                self.move(punto) 
                punto.Fall(Mapa)
            else:
                puntoI = self.PuntoDI
                if self.canLand(puntoI):
                    self.move(puntoI) 
                    puntoI.Fall(Mapa)
                else:
                    puntoD = self.PuntoDR
                    if self.canLand(puntoD):
                        self.move(puntoD) 
                        puntoD.Fall(Mapa)
    def pos(self):
        return str(self.x) + "," + str(self.y)


class Map:
    Mapa = []
    d = {}
    Stop = False
    Cuenta = 0
    def toString(self):
        string = ""
        for columna in self.Mapa:
            for linea in columna:
                string += linea.symbol + ""
            string += "\n"
        return string
    
    def getPunto(self,x,y):
        x = int(x)
        y = int(y)
        for i in self.Mapa:
            for j in i:
                if j.x == x and j.y == y:
                    return j
        return None

    def drawMap(self, minx,x,y):
        nuevaLinea = []
        i = 0
        for linea in range(0,y+3):
            nuevaLinea= []
            for columna in range(minx-170, x +170):
                y = i
                tile = Tile(columna,y)
                self.d[str(columna) + "," + str(y)] = tile
                nuevaLinea.append(tile)
            self.Mapa.append(nuevaLinea)
            i += 1

    def Populate(self,ListaMovs):

        Resultados = []
        for i in range(len(ListaMovs)-1):
            I = ListaMovs[i]
            U = ListaMovs[i+1]
            Ix, Iy = I.split(",")
            Ux, Uy = U.split(",")
            
            if int(Ix) - int(Ux) != 0:
                for i in range(min(int(Ix),int(Ux)), max(int(Ix),int(Ux)) + 1):
                    Resultados.append(str(i) +"," + Iy)
            if int(Iy) - int(Uy) != 0:
                for i in range(min(int(Iy),int(Uy)), max(int(Iy),int(Uy)) + 1):
                    Resultados.append(Ix +"," + str(i))
        for result in Resultados:
            Ix, Iy = result.split(",")
            MapaTiles.d.get(str(Ix).strip()+ "," + str(Iy).strip()).cambiarTipo(bcolors.FAIL + "#" + bcolors.ENDC, "Rock")

    def RenderImage(self,cuenta):
        PixelValues = []
        for linea in self.Mapa:
            nuevalinea = []
            for pixel in linea:
                if pixel.type == "Air" or pixel.type == "Spawner":
                    nuevalinea.append((0,0,0))
                elif pixel.type == "Rock":
                    nuevalinea.append((82,82,82))
                elif pixel.type == "Sand":
                    nuevalinea.append((222, 222, 4))
                else:
                    nuevalinea.append((195, 240, 17))
            PixelValues.append(nuevalinea)
        array = np.array(PixelValues,dtype=np.uint8)
        new_image = Image.fromarray(array)
        path = os.path.dirname(os.path.abspath(__file__))
        new_image.save(path + "/images/" + str(cuenta) + '.png')

    def startSimulation(self):
        while(not self.Stop):
            punto = MapaTiles.d.get("500,0")
            if punto.type != "Sand":
                MapaTiles.d.get("500,0").cambiarTipo(bcolors.WARNING + "O" + bcolors.ENDC, "Sand").Fall(self)
                self.Cuenta += 1
                #print(self.toString())
            else:
                self.Stop = True
            x = threading.Thread(target=self.RenderImage, args=(self.Cuenta,))
            x.start()
        print("Total: " + str(self.Cuenta))

with open("input") as f:
    input = f.readlines()

minx = 500
x = 0
y = 0
MapaTiles = Map()

for linea in input:
    datos = linea.strip().split("->")
    for dato in datos:
        numeros = dato.split(",")
        numx = int(numeros[0])
        numy = int(numeros[1])
        if min(minx, numx) < minx:
            minx = numx
        if max(x, numx) > x:
            x = numx
        if max(y,numy) > y:
            y = numy

MapaTiles.drawMap(minx,x,y)
stringtest = str(minx-170) + "," + str(y+2) + "->" + str(x+169) + "," + str(y+2) + "\n"

print("Mapa generado. Populando y preparando terreno...")

input.append(stringtest)
for linea in input:
    datos = linea.strip().split("->")
    Mov = []
    for dato in datos:
        Mov.append(dato)
    MapaTiles.Populate(Mov)

MapaTiles.getPunto(500,0).cambiarTipo("+", "Spawner")

print("Terreno populado. Preparando casillas...")

for linea in MapaTiles.Mapa:
    for tile in linea:
        puntoD = MapaTiles.d.get(str(tile.x) + "," + str(tile.y+1))
        if puntoD != None:
            tile.PuntoD = puntoD
        puntoDI = MapaTiles.d.get(str(tile.x-1) + "," + str(tile.y+1))
        if puntoDI != None:
            tile.PuntoDI = puntoDI
        puntoDR = MapaTiles.d.get(str(tile.x+1) + "," + str(tile.y+1))
        if puntoDR != None:
            tile.PuntoDR = puntoDR

print("Todos los datos han sido leídos. Iniciando...")

MapaTiles.startSimulation()
