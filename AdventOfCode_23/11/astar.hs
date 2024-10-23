aStar :: [String] -> Par -> Coordenada -> [Coordenada] -> [Coordenada] -> Int
  aStar l ((oy,ox),(gy,gx)) (y,x) checked open = do 
    let newOpen = open ++ openFromPosition l checked (y,x)
        bestCost = getBetterCoord l ((oy,ox),(gy,gx)) newOpen
    if trace((show bestCost) ++ " -> " ++ (show (gy,gx)) ++ " -> " ++ (show $ bestCost == (gy,gx)) ) (bestCost == (gy,gx)) then 1 else 1 + aStar l ((oy,ox),(gy,gx)) bestCost ((y,x):checked) open

  getGCost :: [String] -> Par -> Coordenada -> Int
  getGCost l ((oy,ox),(gy,gx)) (y,x) = ((abs $ y - oy) + (abs $ x - ox))

  getHCost :: [String] -> Par -> Coordenada -> Int
  getHCost l ((oy,ox),(gy,gx)) (y,x) = ((abs $ y - gy) + (abs $ x - gx))
  
  getFCost :: [String] -> Par -> Coordenada -> Int
  getFCost l p c = (getGCost l p c) + (getHCost l p c)

  getBetterCoord :: [String] -> Par -> [Coordenada] -> Coordenada
  getBetterCoord l p coords = head $ sortOn (\ coord -> (getFCost l p coord, getGCost l p coord)) coords

  openFromPosition :: [String] -> [Coordenada] -> Coordenada -> [Coordenada]
  openFromPosition l checked (y,x) = filter (\ c -> c `notElem` checked) [(y+1,x), (y-1,x), (y,x+1), (y,x-1)] 


