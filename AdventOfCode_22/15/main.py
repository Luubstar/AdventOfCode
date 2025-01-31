ySeleccionada = 10

class Sensor:
    posicion = ()
    BeaconPos = ()

    def __init__(self,x,y,beaconpos) -> None:
        self.posicion = (int(x),int(y))
        self.BeaconPos = beaconpos
    
    def CalcularDistancia(Punto1, Punto2):
        return abs(Punto1[0]- Punto2[0]) + abs(Punto1[1] - Punto2[1])   



with open("input") as f:
    inputs = f.readlines()

Sensores = []
SensoresPosiciones = set()

for linea in inputs:
    linea = linea.replace("Sensor at ", "").replace(" closest beacon is at ","").replace("x=","").replace("y=","").strip().split(":")
    sensorpos,beaconpos = zip(linea)
    sensorpos = sensorpos[0].split(",")
    beaconpos = beaconpos[0].split(",")
    beacon = Sensor(beaconpos[0],beaconpos[1],None)
    sensor = Sensor(sensorpos[0],sensorpos[1],beacon.posicion)
    Sensores.append(Sensor(sensorpos[0],sensorpos[1],beacon.posicion))
    SensoresPosiciones.add((sensor.posicion[0],sensor.posicion[1]))
    SensoresPosiciones.add((beacon.posicion[0],beacon.posicion[1]))


Posiciones = set()
print("Generando posiciones...")
for sensor in Sensores:
    distancia = Sensor.CalcularDistancia(sensor.posicion, sensor.BeaconPos)
    yarriba = sensor.posicion[1] - distancia
    yabajo = sensor.posicion[1] + distancia

    if yarriba <= ySeleccionada and sensor.posicion[1] >= ySeleccionada:
        for i in range(0, abs(yarriba - ySeleccionada) + 1 ):
            Posiciones.add(sensor.posicion[0] + i)
            Posiciones.add(sensor.posicion[0] - i)

    if yabajo >= ySeleccionada and sensor.posicion[1] <= ySeleccionada:
        for i in range(0, abs(yabajo - ySeleccionada) + 1 ):
            Posiciones.add(sensor.posicion[0] + i)
            Posiciones.add(sensor.posicion[0] - i)

Cuenta = 0
for punto in Posiciones:
    if not SensoresPosiciones.__contains__((punto,ySeleccionada)):
        Cuenta += 1

print(Cuenta, " Parte 1")

def merge_ranges(ranges):
    ranges.sort(key=lambda x: x[0])
    merged_ranges = []
    for i in range(len(ranges) - 1):
        r1 = ranges[i]
        r2 = ranges[i + 1]
        if r2[0] <= r1[1] <= r2[1] or r2[0] == r1[1] + 1:
            ranges[i + 1] = [r1[0], r2[1]]
        elif r1[0] <= r2[0] and r1[1] >= r2[1]:
            ranges[i + 1] = r1
            continue
        else:
            merged_ranges.append(r1)
    merged_ranges.append(ranges[-1])
    return merged_ranges



for row in range(4000001):
    Posiciones = []
    for sensor in Sensores:
        distancia = Sensor.CalcularDistancia(sensor.posicion, sensor.BeaconPos)
        distanciaR = distancia - abs(row - sensor.posicion[1])
        if distanciaR < 0:
            continue
        Posiciones.append([sensor.posicion[0] - distanciaR, sensor.posicion[0] + distanciaR])
    PosicionesRangos = merge_ranges(Posiciones)
    if len(PosicionesRangos) > 1:
        print((PosicionesRangos[0][1] + 1) * 4000000 + row)
        exit()