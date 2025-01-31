import os
Directorios = []
class Directorio():

    Nombre = ""
    DirectorioOrigen = 0
    Archivos = []
    Directorios = []
    PesoTotal = 0

    def __init__(self,nombre, DirectorioPadre):
        self.Nombre = nombre
        self.DirectorioOrigen = DirectorioPadre
    
    def AddFile(self, file):
        self.Archivos.append(file)
    
    def AddDirectorio(self,dir):
        self.Directorios.append(dir)

    def ContarPeso(self):
        self.PesoTotal = 0
        peso = 0
        for directorio in self.Directorios:
            peso += directorio.ContarPeso()
        for file in self.Archivos:
            if file.Directorio == self:
                peso += int(file.peso)
        self.PesoTotal = peso
        return peso

    def PrintContent(self):
        self.ContarPeso()
        print("\t"+ self.Nombre + "(" + str(self.PesoTotal) + ") :")

        for file in self.Archivos:
            if file.Directorio == self:
                print("\t\t-> " + file.Nombre + "(" + str(file.peso) + ")")

        for directorio in self.Directorios:
            directorio.PrintContent()

    def PrintAll(self,modo = "dir"):
        if modo == "dir":
            for directorio in Directorios:
                directorio.ContarPeso()
                print(directorio.Nombre + "(" + str(directorio.PesoTotal) + ")")
                
        elif modo == "All":
            print("- /")
            self.PrintContent()
            
        elif modo == "cuenta":
            cuenta = 0
            for directorio in Directorios:
                directorio.ContarPeso()
                if directorio.PesoTotal <= 100000:
                    cuenta += directorio.PesoTotal
            print(cuenta)
            return cuenta

    def GetByName(self,nombre):
        for dir in self.Directorios:
            if dir.Nombre == nombre:
                return dir
        

class Archivo():

    Nombre = ""
    Directorio = None
    peso = 0

    def __init__(self, nombre, Directorio, peso):
        self.Nombre = nombre
        self.Directorio = Directorio
        self.peso = peso



DirectorioRoot = Directorio("/", None)
DirectorioActual = DirectorioRoot

with open(os.getcwd() + "/input") as f:
    input = list(f.read().split("\n"))
    input.remove(input[0])
    for linea in input:
        
        if linea.startswith("$"):
            if linea.__contains__("cd"):
                nombre = linea.replace("$ cd", "").strip()
                if nombre == "..":
                    DirectorioActual = DirectorioActual.DirectorioOrigen
                else:
                    dir = DirectorioActual.GetByName(nombre)
                    DirectorioActual = dir
        else:
            if linea.startswith("dir"):
                nombre = linea.replace("dir", "").strip()
                dir = Directorio(nombre,DirectorioActual)
                DirectorioActual.AddDirectorio(dir)
                dir.Directorios = []
                Directorios.append(dir)
            else:
                file = linea.split(" ")
                DirectorioActual.AddFile(Archivo(file[1], DirectorioActual, file[0]))


filespace = 70000000
necesario = 30000000
usado = DirectorioRoot.ContarPeso()
DirectorioRoot.PrintAll("All")
libre = filespace - usado
requerido = necesario - libre
Posibles = []
for directorio in Directorios:
    if directorio.PesoTotal >= requerido:
        Posibles.append(directorio)

min = 300000000
directSelect = None
for dir in Posibles:
    if dir.PesoTotal < min:
        min = dir.PesoTotal
        directSelect = dir
print(libre, requerido, directSelect.Nombre,directSelect.PesoTotal)