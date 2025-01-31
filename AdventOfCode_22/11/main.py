import math
Monos = []
modulo = 1

class mono:
    nombre = 0
    operacion = ""
    test = 0
    mono_True = 0
    mono_False = 0
    inspected = 0

    def __init__(self, nombre, operacion, inventario, test, mono_true, mono_false) -> None:
        self.inventario = []
        self.nombre = nombre
        self.operacion = operacion
        self.inventario.append([i for i in inventario.split(",")])
        self.inventario = self.inventario[0]
        self.test = test
        self.mono_True = mono_true
        self.mono_False = mono_false
        Monos.append(self)

    def operar(self, item):
        return eval(str(self.operacion.replace("old", str(item)))) % modulo

    def toString(self):
        string = """
        Monkey {nombre}:
        Items: {inventario}
        Operation: {operacion}
        Test: divisible por {test}
            if true: to {mono_True}
            if false: to {mono_False}
        Inspeccionó: {inspected}"""
        print(string.format(nombre = self.nombre, inventario = self.inventario,
         operacion = self.operacion, test = self.test, mono_True = self.mono_True,
          mono_False = self.mono_False, inspected = self.inspected))
    
    def ronda():
        for mono in Monos:
            for item in mono.inventario:
                mono.inspected += 1
                new = math.floor(mono.operar(item))

                if new % int(mono.test) == 0:
                    Monos[int(mono.mono_True)].inventario.append(new)
                else:
                    Monos[int(mono.mono_False)].inventario.append(new)
            mono.inventario.clear()
           

with open("input") as f:
    input = f.read().split("\n\n")
    
    i = 0
    for lineas in input:
        lineas = lineas.split("\n")
        inventario = lineas[1].replace("Starting items:", "").strip()
        operacion = lineas[2].replace("Operation: new = ","").strip()
        test = lineas[3].replace("Test: divisible by ","").strip()
        true = lineas[4].replace("If true: throw to monkey ","").strip()
        false = lineas[5].replace("If false: throw to monkey", "").strip()
        monoNuevo = mono(i,operacion,inventario,test,true,false)
        i += 1

    
    for m in Monos:
        modulo *= int(m.test)

    for i in range(10000):
        print(i)
        mono.ronda()

inspecciones = []
for monkie in Monos:
    print(monkie.toString())
    inspecciones.append(monkie.inspected)

inspecciones.sort()

print(inspecciones[-1] * inspecciones[-2])
