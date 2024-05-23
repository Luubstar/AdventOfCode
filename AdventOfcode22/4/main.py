import os
cuenta = 0
with open(os.getcwd() + "/input") as f:
    pareja = f.read().split("\n")
    pareja.remove(pareja[pareja.__len__()-1])
    for duo in pareja:
        elfos = duo.split(",")
        zonasElfo1 = []
        zonasElfo2 = []

        elfo1 = elfos[0]
        elfo2 = elfos[1]
        
        for i in range(int(elfo1.split("-")[0]), int(elfo1.split("-")[1]) + 1):
            zonasElfo1.append(i)
        for i in range(int(elfo2.split("-")[0]), int(elfo2.split("-")[1]) + 1):
            zonasElfo2.append(i)
    
        contains = False

        for zona in zonasElfo1:
            if zonasElfo2.__contains__(zona):
                contains = True
        
        if contains:
            cuenta += 1
        else:
            contains = False
            for zona in zonasElfo2:
                if zonasElfo1.__contains__(zona):
                    contains = True
            
            if contains:
                cuenta += 1
print(cuenta)
        