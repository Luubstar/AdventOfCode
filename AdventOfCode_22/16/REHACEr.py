from collections import defaultdict

class Valvula:
    Nombre = "00"
    Conexiones = []
    Ritmo = 0
    Activada = False
    abierta = 0

    Valvulas = []
    ValvulasDict = []

    def __init__(self, nombre, ritmo, conecta) -> None:
        self.Nombre = nombre
        self.Conexiones = conecta
        self.Ritmo = int(ritmo)
    
    def toString(self):
        return "Nombre: " + self.Nombre + " Ritmo: " + str(self.Ritmo) + " Conecta con: " + str(self.Conexiones)
    
    @staticmethod
    def GenerarGrafo():
        Grafo = []
        for valvula in Valvula.Valvulas:
            for conexion in valvula.Conexiones:
                Grafo.append([valvula.Nombre, conexion])
        graph = defaultdict(list)
        for edge in Grafo:
            a, b = edge[0], edge[1]
            graph[a].append(b)
            graph[b].append(a)
        return graph

    def find_distances(rooms):
        key_rooms = {r for r in rooms if r.Ritmo > 0 or r.Nombre == "AA"}
        distances = {}

        for start_room in rooms:
            if start_room not in key_rooms: continue
            # perform a BFS starting in start_room, recording all distances
            cur, next, dist = set([
                start_room,
            ]), set(), 0
            distances[(start_room, start_room)] = 0
            while cur:
                dist += 1
                for pos in cur:
                    for newpos in rooms[pos].exits:
                        if (start_room, newpos) not in distances:
                            distances[(start_room, newpos)] = dist
                            next.add(newpos)
                cur, next = next, set()

        return distances, key_rooms

    @staticmethod
    def findByName(name):
        for valv in Valvula.Valvulas:
            if str(valv.Nombre).strip() == name.strip():
                return valv
        return None
    
    @staticmethod
    def getSiguiente(Actual, minutosRestantes): #A cambiar. Calcula el DFS
        Cargados = []
        Posibles =[]
        for i in Actual.Conexiones:
            Posibles.append(i)
        Valores = dict()
        if not Actual.Activada and Actual.Ritmo > 0:
            Valores[Actual.Nombre] = Actual.Ritmo
        distancia = 1
        while(len(Posibles) > 0):
            Aux = []
            for valvNombre in Posibles:
                valv = Valvula.findByName(valvNombre)
                if Valores.get(valv.Nombre) == None and valv.Activada == False:
                    Valores[valv.Nombre] = valv.Ritmo / (distancia)
                    for conexion in valv.Conexiones:
                        Aux.append(conexion)
                if valv.Nombre not in Cargados:
                    for conexion in valv.Conexiones:
                        Aux.append(conexion)
            Posibles.clear()
            for aux in Aux:
                Posibles.append(aux)
                Cargados.append(aux)
            distancia += 1
        return Valores

Grafo = None
with open("input") as f:
    entradas = f.readlines()

for entrada in entradas:
    entrada = (
        entrada.replace("Valve", "").replace("has flow ","").replace("tunnels lead to valves ", "").replace("tunnel leads to valve ", "").strip()
    )
    valvula = entrada.split(";")
    datos1 = valvula[0].split("rate=")
    datos2 = valvula[1].split(",")
    conex = []
    for dato in datos2:
        conex.append(dato.strip())
    Valvula.Valvulas.append(Valvula(datos1[0].strip(),datos1[1], conex))

Actual = Valvula.findByName("AA")
Grafo = Valvula.GenerarGrafo()
presionFinal = 0
ritmo = 0
ritmobus = 0
print(Valvula.find_distances(Valvula.Valvulas))
print(presionFinal)