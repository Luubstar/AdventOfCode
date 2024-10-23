module Dia16 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
  
  import Data.List (elem)
  import Debug.Trace (trace)
  import qualified Data.Set as Set

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

  type Coordenada = (Int, Int)
  type Direccion = (Int, Int)
  type Dato = (Coordenada, Direccion)
 
-- ##############################################
-- ## Función main                             ##
-- ##############################################

  test = do 
   contents <- (readFile "Test.txt")
   let lin = lines contents
      
   putStrLn $ show $ part1 lin
   putStrLn "--------"
   putStrLn $ show $ part2 lin 

  main = do
   contents <- (readFile "Input16.txt")
   let lin = lines contents
      
   putStrLn $ show $ part1 lin
   putStrLn "--------"
   putStrLn $ show $ part2 lin 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: [String] -> Int
  part1 l = 
    let pos = countPos [] $ Set.elems $ move l (listToSet [])  (-1,0) (1,0) 
    in length pos

  move :: [String] -> Set.Set Dato -> Coordenada -> Direccion -> Set.Set Dato
  move l datos (x,y) (dx,dy) 
   | Set.member thiscoord datos || nx < 0 || ny < 0 || ny >= length l || nx >= length (l!!0)  = datos
   | charat == '/'  = move l (Set.insert thiscoord datos) (nx,ny) (dy*(-1),dx*(-1))
   | charat == '\\' = move l (Set.insert thiscoord datos) (nx,ny) (dy, dx)
   | charat == '-' && dy /= 0 = 
     let iz = move l (Set.insert thiscoord datos) (nx,ny) (-1,0)
         dr = move l iz (nx,ny) (1,0)
     in dr
   | charat == '|' && dx /= 0 = 
     let up = move l (Set.insert thiscoord datos) (nx,ny) (0,-1)
         dn = move l up (nx,ny) (0,1)
      in dn
   | otherwise = move l (Set.insert thiscoord datos) (nx,ny) (dx,dy)
   where 
    nx = x+dx 
    ny = y+dy    
    thiscoord =  ((nx,ny),(dx,dy))
    charat = (l!!ny)!!nx

  countPos :: [Coordenada] -> [Dato] -> [Coordenada]
  countPos carry (((x,y),(_,_)):datos) = if (not $ elem (x,y) carry) then countPos ((x,y):carry) datos else countPos carry datos
  countPos c [] = c

-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

  part2 :: [String] -> Int 
  part2 l = maximum $ moveAll l $ genPositions l 0 0 

  moveAll :: [String] ->  [Dato] -> [Int] 
  moveAll l (((x,y),(dx,dy)):datos) = 
   let pos = countPos [] $ Set.elems $ move l (listToSet [])  (x,y) (dx,dy)
   in [length pos] ++ moveAll l datos
  moveAll _ [] = []

  genPositions :: [String] -> Int -> Int -> [Dato] 
  genPositions l x y 
   | x == (length (l!!0))- 1 = [(((x+1),y),(-1,0))] ++ genPositions l 0 (y+1) 
   | y == (length l) - 1 = [((x,(y+1)),(0,-1))] ++ genPositions l (x+1) y 
   | x == 0 = [(((x-1),y),(1,0))] ++ genPositions l (x+1) y 
   | y == 0 = [((x,(y-1)),(0,1))] ++ genPositions l (x+1) y
   | y >= length l = [] 
   | otherwise = genPositions l (x+1) y 

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################
 
  listToSet :: Ord a => [a] -> Set.Set a
  listToSet = Set.fromList
