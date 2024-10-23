module Dia10 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
  import Data.List (elemIndex, elem)
  import Debug.Trace (trace)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

  type Coordenada = (Int, Int)

  pipes = "LF7J"

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input10.txt")
   let lin = lines contents
       cInicial = findStart lin 0
   
   putStrLn $ show $ part1 lin cInicial cInicial (0,0) 0
   putStrLn "--------"
   putStrLn $ show $ part2 lin cInicial cInicial 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: [String] -> Coordenada -> Coordenada -> Coordenada -> Int -> Int 
  part1 lista cInicial cActual inertia pasos 
   | pasos > 0 && cInicial == cActual = div pasos 2 
   | otherwise = 
      let newpos   = (move lista cActual inertia)
          movement =diff cActual newpos 
      in part1 lista cInicial newpos movement (pasos+1)

  move :: [String] -> Coordenada -> Coordenada -> Coordenada 
  move lista (y,x) (y1,x1) = 
   let letra = getPos lista (y,x)
   in case letra of 
     'S' -> firstmove lista (y,x)
     '7' | y1 == 1 -> (y, x-1)
         | x1 == (-1) -> (y+1, x)
     'J' | x1 == (-1) -> (y-1, x)
         | y1 == (-1) -> (y, x-1)
     'L' | y1 == (-1) -> (y, x+1)
         | x1 == 1 -> (y-1,x)
     'F' | y1 == 1 -> (y, x+1)
         | x1 == 1 -> (y+1, x)
     '|' -> (y-y1, x)
     '-' -> (y, x-x1)
     _ -> error $ show (y1,x1)

  firstmove :: [String] -> Coordenada -> Coordenada
  firstmove lista (y,x) 
   | getPos lista (y,x+1) == '-' || getPos lista (y,x+1) == 'J' || getPos lista (y,x+1) == '7' = (y, x+1)
   | getPos lista (y+1,x) == '|' || getPos lista (y+1,x) == 'F' || getPos lista (y+1,x) == '7' = (y+1, x)
   | getPos lista (y-1,x) == '|' || getPos lista (y-1,x) == 'J' || getPos lista (y-1,x) == 'L' = (y-1, x)
   | otherwise = (y,x-1)


-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################
  part1' :: [String] -> Coordenada -> Coordenada -> Coordenada -> [Coordenada] -> [Coordenada] 
  part1' lista cInicial cActual inertia vertex
   | (length vertex) > 2 && cInicial == cActual = vertex
   | otherwise = 
      let newpos   = (move lista cActual inertia)
          movement =  diff cActual newpos 
      in if elem (getPos lista newpos) pipes then part1' lista cInicial newpos movement (newpos:vertex) else part1' lista cInicial newpos movement vertex

  part2 :: [String] -> Coordenada -> Coordenada -> Int 
  part2 lista cInicial cActual =  
    let vertices = part1' lista cInicial cInicial (0,0) [cInicial]
        puntos = getGround lista 0 0
        dentro = ( abs $ (area vertices) - (part1 lista cInicial cInicial (0,0) 0) + 1)
    in dentro 
   
-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

  findStart :: [String] -> Int -> Coordenada 
  findStart (x:xs) pasos
   | elem 'S' x = do 
     case elemIndex 'S' x of
      Just t -> (pasos, t)
      _ -> error "WTF?"
   | otherwise = findStart xs (pasos + 1)
  findStart [] _ = error "Inicio no encontrado"

  getGround :: [String] -> Int -> Int -> [Coordenada]
  getGround (y:ys) x ypos
   | x >= length y = getGround ys 0 (ypos+1)
   | otherwise = [(ypos,x)] ++ getGround (y:ys) (x+1) ypos 
  getGround [] _ _ = [] 

  getPos :: [String] -> Coordenada -> Char 
  getPos l (y,x) = (l!!y)!!x

  diff :: Coordenada -> Coordenada -> Coordenada 
  diff (x,y) (x1,y1) = (x-x1, y-y1)

  area :: [Coordenada] -> Int 
  area l = (abs $ shoeLace (l ++ [head l])) `div` 2 

  shoeLace :: [Coordenada] -> Int 
  shoeLace [_] = 0 
  shoeLace ((y1, x1) : (y2, x2) : xs) = (y1 - y2) * (x2 + x1) + shoeLace ((y2, x2) : xs)

