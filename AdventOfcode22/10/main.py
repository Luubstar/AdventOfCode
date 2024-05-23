input = []

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


class CPU():
    x = 1
    ciclo = 1
    valor = 0
    def __init__(self):
        self.x = 1
        self.ciclo = 0
    def Orden(self, orden, valor = 0):
        if orden == "noop":
            self.aumentarCiclo()
        elif orden == "addx":
            self.aumentarCiclo()
            self.x += valor
            self.aumentarCiclo()
    def aumentarCiclo(self):
        self.ciclo += 1
        self.Print()
    def Print(self):
        

        if self.ciclo % 40 == 0:
            print("\n", end="")
        if self.ciclo%40 == self.x or self.ciclo%40 == self.x + 1 or self.ciclo%40 == self.x -1:
            print(f"{bcolors.WARNING}#{bcolors.ENDC}",end="")
        else:
            print(f"{bcolors.OKGREEN}.{bcolors.ENDC}",end="")
      



with open("input") as f:
    input = f.readlines()

cpu = CPU()
for linea in input:
    if linea.startswith("noop"):
        cpu.Orden("noop")
    else:
        linea = linea.replace("\n","").split(" ")
        cpu.Orden(linea[0],int(linea[1]))
    
print("\n")