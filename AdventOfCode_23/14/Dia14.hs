module Dia14 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
  
  import Data.List (sortOn,transpose, elemIndex)
  import Debug.Trace (trace)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 
 
  type Coordenada = (Int,Int)
  type RocaCuadrada = Coordenada 
  type RocaCircular = Coordenada

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input14.txt")
   let lin = lines contents

   putStrLn $ show $ part1 lin
   putStrLn "--------"
   putStrLn $ show $ part2 lin 0 [] 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: [String] -> Int
  part1 lin =  
   let rocas = transpose $ map (findRock 0) (transpose $ lin)
   in countRock (length rocas) rocas 

  findRock :: Int -> String -> String
  findRock x l
   | x >= length l = l
   | l!!x == '#' || l!!x == '.' = findRock (x+1) l
   | otherwise = findRock (x+1) (moveRock l (x-1) x)

  moveRock :: String -> Int -> Int -> String
  moveRock l x fx 
   | x < 0 = take 0 l ++ ['O'] ++ [if i == fx then '.' else j | (i,j) <- zip [1..] (drop (1) l)]
   | l!!x == '#' || l!!x == 'O' = take (x+1) l ++ ['O'] ++ [if i == fx then '.' else j | (i,j) <- zip [x+2..] (drop (x+2) l)]
   | otherwise = moveRock l (x-1) fx
   
-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

  part2 :: [String] -> Int -> [[String]] -> Int 
  part2 l x lista
   | (length lista) > 0 && elem l (init lista) = 
     let indexciclo = getIndex l (init lista) 
         listam = drop (indexciclo) (init lista)
     in countRock (length l) (listam!!((mod (1000000000 - indexciclo - 1) (length $ listam))))
   | x >= 1000000000 = countRock  (length l) l
   | otherwise = 
     let l1 = findRockN l 
         l2 = findRockW l1
         l3 = findRockS l2
         l4 = findRockE l3 
         in part2 (l4) (x+1) (lista++[l4])

  findRockN ::  [String] -> [String] 
  findRockN l = transpose $ map (findRock 0) (transpose $ l)

  findRockE ::  [String] -> [String] 
  findRockE l = map reverse $ map (findRock 0) (map reverse $ l)

  findRockS ::  [String] -> [String] 
  findRockS l = transpose $ map reverse $ map (findRock 0) (map reverse $ transpose $ l)

  findRockW ::  [String] -> [String] 
  findRockW l = map (findRock 0) l

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

  countRock :: Int -> [String] -> Int 
  countRock y ((x:xs):ys)  
   | x == 'O' = y + countRock y ((xs):ys) 
   | otherwise = countRock y ((xs):ys) 
  countRock y ([]:ys) = countRock (y-1) ys 
  countRock _ [] = 0
 
  getIndex :: [String] -> [[String]] -> Int 
  getIndex l s = 
   case elemIndex l s of
    Just t -> t
    _ -> -1
