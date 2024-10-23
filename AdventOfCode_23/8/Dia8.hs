module Dia8 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
  import Debug.Trace (trace)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

  type Instrucciones = String
  data Nodo = Nodo String String String deriving Show 

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input8.txt")
   let lin = lines contents

   putStrLn $ show $ part1 (head lin) (head lin) (getNode "AAA" $ parseNode $ drop 2 lin)  (parseNode $ drop 2 lin) 
   putStrLn "--------"
   putStrLn $ show $ part2 (head lin)  (parseNode $ drop 2 lin) (getNodeThatEnds $ parseNode $ drop 2 lin)  
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: Instrucciones -> Instrucciones -> Nodo ->  [Nodo] -> Int
  part1 (x:xs) i (Nodo tag l r) nodos 
   | tag == "ZZZ" = 0 
   | otherwise = 1 + part1 xs i (moveNode x (Nodo tag l r) nodos) nodos
  part1 [] s a n = 0 + part1 s s a n

  moveNode :: Char -> Nodo -> [Nodo] -> Nodo 
  moveNode i (Nodo tag l r) nodos
   | i == 'L' = getNode l nodos 
   | i == 'R' = getNode r nodos

-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################
  
  part2 :: Instrucciones -> [Nodo] ->  [Nodo] -> Int
  part2 i nodos nodosBuscar =  mcmLista  $ map (part1' i i nodos) nodosBuscar  
  
  part1' :: Instrucciones -> Instrucciones ->  [Nodo] -> Nodo -> Int
  part1' (x:xs) i nodos (Nodo tag l r)
   | last tag == 'Z' = 0 
   | otherwise = 1 + part1' xs i nodos (moveNode x (Nodo tag l r) nodos) 
  part1' [] s a n = 0 + part1' s s a n

  mcmLista :: [Int] -> Int
  mcmLista = foldl1 lcm

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################


  parseNode :: [String] -> [Nodo]
  parseNode (x:xs) = [Nodo (head $ words x) (init $ tail $ head $ drop 2 $ words x)  (init $ head $ drop 3 $ words x)] ++ parseNode xs
  parseNode [] = []

  getNode :: String -> [Nodo] -> Nodo 
  getNode c ((Nodo cadena l r):xs) 
   | c == cadena = (Nodo cadena l r)
   | otherwise = getNode c xs
  getNode c [] = error ("WTF " ++ c) (Nodo "" "" "")

  getNodeThatEnds :: [Nodo] -> [Nodo] 
  getNodeThatEnds ((Nodo cadena l r):xs) 
   | (last cadena) == 'A' = [(Nodo cadena l r)] ++ (getNodeThatEnds xs) 
   | otherwise = getNodeThatEnds xs
  getNodeThatEnds [] = []


