module Dia11 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
  import Debug.Trace (trace)
  import Data.List (sortOn)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 
  
  type X = Int 
  type Y = Int
  type Coordenada = (Y, X)
  type Par = (Coordenada, Coordenada)

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input11.txt")
   let lin = lines contents
       expanded =  defaultExpandList lin 1
       moreExpanded = defaultExpandList lin (1000000-1)
   
   putStrLn $ show $ part1 $ getPares expanded expanded
   putStrLn "--------"
   putStrLn $ show $ part1 $ getPares moreExpanded moreExpanded
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################
 
  part1 :: [Par] -> Int 
  part1 (p:xs) = distance p + (part1  xs)
  part1 [] = 0

  distance :: Par -> Int 
  distance ((oy,ox),(gy,gx)) = (abs $ oy-gy) + (abs $ ox - gx)
 
-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################



-- ##############################################
-- ## Funciones generales                      ##
-- ############################################## 

  getColumn :: [String] -> Int -> String
  getColumn (x:xs) col = ((x!!col):getColumn xs col)
  getColumn [] _ = []

  expandList :: [String] -> [String] -> Int -> Int -> Int -> Int -> Int -> [Coordenada]
  expandList (linea:lineas) l xf yf xl yl ancho
   | xf >= (length linea) || not (elem '#' linea) = if (elem '#' linea) then expandList lineas l 0 (yf+1) 0 (yl+1) ancho else expandList lineas l 0 (yf+1) 0 (yl+ancho+1) ancho
   | (linea!!xf) == '#' = [(xl,yl)] ++ expandList (linea:lineas) l (xf+1) yf (xl+1) yl ancho 
   | not (elem '#' (getColumn l xf)) = expandList (linea:lineas) l (xf+1) yf (xl+1+ancho) yl ancho  
   | otherwise = expandList (linea:lineas) l (xf+1) yf (xl+1) yl ancho
  expandList [] _ _ _ _ _ _ = []

  defaultExpandList :: [String] -> Int -> [Coordenada]
  defaultExpandList l ancho = expandList l l 0 0 0 0 ancho

  getPares :: [Coordenada] -> [Coordenada] -> [Par]
  getPares (x:xs) (y:ys) 
   |  x /= y = [(x,y)] ++ getPares (x:xs) (ys)
   | otherwise = getPares (x:xs) (ys)
  getPares (x:xs) [] = getPares xs xs 
  getPares [] _ = []
